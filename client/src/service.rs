use crate::control::{Control, SessionEnd};
use crate::handle::ClientStatus;
use crate::run_id;
use anyhow::{anyhow, Result};
use orbien_core::config::ClientConfig;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;
use tokio_util::sync::CancellationToken;

pub struct Service {
    cfg: ClientConfig,
    config_path: PathBuf,
}

impl Service {
    pub fn new(cfg: ClientConfig, config_path: impl Into<PathBuf>) -> Self {
        Self {
            cfg,
            config_path: config_path.into(),
        }
    }

    pub async fn run(
        self,
        cancel: CancellationToken,
        mut on_status: impl FnMut(ClientStatus),
        mut on_log: impl FnMut(String),
        on_proxy_remote: Arc<dyn Fn(String, String) + Send + Sync>,
        on_remotes_clear: Arc<dyn Fn() + Send + Sync>,
    ) -> Result<()> {
        let mut run_id = run_id::load(&self.config_path);
        if !run_id.is_empty() {
            tracing::info!(%run_id, "restored persisted run_id");
        }

        let mut first_attempt = true;
        loop {
            if cancel.is_cancelled() {
                tracing::info!("client service cancelled");
                return Ok(());
            }

            on_remotes_clear();

            if first_attempt {
                on_status(ClientStatus::Starting);
                on_log("INFO  connecting to server".into());
            } else {
                on_status(ClientStatus::Reconnecting);
            }

            let end = Control::start(
                &self.cfg,
                run_id.clone(),
                &self.config_path,
                cancel.clone(),
                || {
                    on_status(ClientStatus::Running);
                    on_log("INFO  connected to server".into());
                },
                Arc::clone(&on_proxy_remote),
            )
            .await;

            on_remotes_clear();

            match end {
                Ok(SessionEnd::Kicked {
                    run_id: rid,
                    reason,
                }) => {
                    tracing::error!(
                        run_id = %rid,
                        %reason,
                        "kicked by server — stopping (no reconnect)"
                    );
                    on_log(format!("ERROR kicked by server: {reason}"));
                    return Err(anyhow!("kicked by server: {reason}"));
                }
                Ok(SessionEnd::Disconnected { run_id: rid }) => {
                    if cancel.is_cancelled() {
                        tracing::info!(run_id = %rid, "session ended after cancel");
                        return Ok(());
                    }
                    run_id = rid;
                    on_log("WARN  disconnected from server".into());
                    on_status(ClientStatus::Reconnecting);
                }
                Err(e) => {
                    if cancel.is_cancelled() {
                        tracing::info!("session error after cancel: {e}");
                        return Ok(());
                    }
                    on_log(format!("ERROR failed to connect: {e}"));
                    on_status(ClientStatus::Reconnecting);
                }
            }

            first_attempt = false;
            on_log("INFO  retrying in 3s".into());

            tokio::select! {
                _ = cancel.cancelled() => {
                    tracing::info!("client service cancelled during backoff");
                    return Ok(());
                }
                _ = sleep(Duration::from_secs(3)) => {}
            }
        }
    }
}
