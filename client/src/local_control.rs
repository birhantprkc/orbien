use crate::handle::ClientHandle;
use crate::reload::ReloadOutcome;
use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{UnixListener, UnixStream};
use tokio_util::sync::CancellationToken;

#[derive(Debug, Deserialize)]
struct ControlRequest {
    op: String,
    #[serde(default)]
    config: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ControlResponse {
    ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    outcome: Option<ReloadOutcome>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

pub fn default_socket_path() -> PathBuf {
    if let Some(dir) = std::env::var_os("XDG_RUNTIME_DIR") {
        return PathBuf::from(dir).join("orbien").join("control.sock");
    }
    dirs_fallback().join("run").join("control.sock")
}

fn dirs_fallback() -> PathBuf {
    if let Some(home) = dirs_home() {
        #[cfg(target_os = "macos")]
        {
            return home
                .join("Library")
                .join("Application Support")
                .join("com.orbien.client");
        }
        #[cfg(not(target_os = "macos"))]
        {
            return home.join(".config").join("orbien");
        }
    }
    PathBuf::from("/tmp/orbien")
}

fn dirs_home() -> Option<PathBuf> {
    std::env::var_os("HOME").map(PathBuf::from)
}

pub async fn serve(
    socket_path: PathBuf,
    handle: ClientHandle,
    cancel: CancellationToken,
) -> Result<()> {
    if let Some(parent) = socket_path.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("create socket dir {}", parent.display()))?;
    }
    if socket_path.exists() {
        let _ = std::fs::remove_file(&socket_path);
    }

    let listener = UnixListener::bind(&socket_path)
        .with_context(|| format!("bind unix socket {}", socket_path.display()))?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&socket_path, std::fs::Permissions::from_mode(0o600))?;
    }

    tracing::info!(path = %socket_path.display(), "local control socket listening");

    loop {
        tokio::select! {
            _ = cancel.cancelled() => break,
            accepted = listener.accept() => {
                let (stream, _) = accepted?;
                let hc = handle.clone();
                tokio::spawn(async move {
                    if let Err(e) = handle_connection(stream, hc).await {
                        tracing::debug!(error = %e, "control connection ended");
                    }
                });
            }
        }
    }

    let _ = std::fs::remove_file(&socket_path);
    Ok(())
}

async fn handle_connection(stream: UnixStream, handle: ClientHandle) -> Result<()> {
    let (reader, mut writer) = stream.into_split();
    let mut lines = BufReader::new(reader).lines();
    let line = match lines.next_line().await? {
        Some(l) if !l.trim().is_empty() => l,
        _ => return Err(anyhow!("empty control request")),
    };

    let req: ControlRequest = serde_json::from_str(&line)?;
    let resp = match req.op.as_str() {
        "reload" => handle_reload(&handle, req.config.as_deref()).await,
        "ping" => ControlResponse {
            ok: true,
            outcome: None,
            error: None,
        },
        other => ControlResponse {
            ok: false,
            outcome: None,
            error: Some(format!("unknown op: {other}")),
        },
    };

    let body = serde_json::to_string(&resp)?;
    writer.write_all(body.as_bytes()).await?;
    writer.write_all(b"\n").await?;
    Ok(())
}

async fn handle_reload(handle: &ClientHandle, config: Option<&str>) -> ControlResponse {
    let path = match config {
        Some(p) => PathBuf::from(p),
        None => {
            return ControlResponse {
                ok: false,
                outcome: None,
                error: Some("config path is required".into()),
            };
        }
    };
    match handle.reload_from_path(&path).await {
        Ok(outcome) => ControlResponse {
            ok: outcome.succeeded(),
            outcome: Some(outcome),
            error: None,
        },
        Err(e) => ControlResponse {
            ok: false,
            outcome: None,
            error: Some(e.to_string()),
        },
    }
}

pub async fn reload_via_socket(config_path: &Path) -> Result<ReloadOutcome> {
    let stream = UnixStream::connect(default_socket_path())
        .await
        .context("connect to running client (start orbien with -c first)")?;
    let (reader, mut writer) = stream.into_split();
    let req = serde_json::json!({
        "op": "reload",
        "config": config_path.to_string_lossy(),
    });
    writer.write_all(req.to_string().as_bytes()).await?;
    writer.write_all(b"\n").await?;

    let mut lines = BufReader::new(reader).lines();
    let line = lines
        .next_line()
        .await?
        .ok_or_else(|| anyhow!("empty control response"))?;
    let resp: ControlResponse = serde_json::from_str(&line)?;
    if let Some(outcome) = resp.outcome {
        if resp.ok {
            Ok(outcome)
        } else {
            Err(anyhow!(
                "reload completed with errors: {:?}",
                outcome.failed
            ))
        }
    } else {
        Err(anyhow!(resp
            .error
            .unwrap_or_else(|| "reload failed".into())))
    }
}
