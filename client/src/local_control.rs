use crate::handle::ClientHandle;
use crate::reload::ReloadOutcome;
use anyhow::{anyhow, bail, Context, Result};
use serde::{Deserialize, Serialize};
use std::io;
use std::path::{Path, PathBuf};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{UnixListener, UnixStream};
use tokio_util::sync::CancellationToken;

const ENV_CONTROL_SOCKET: &str = "ORBIEN_CONTROL_SOCKET";
const ENV_STATE_DIR: &str = "ORBIEN_STATE_DIR";
const SOCKET_FILE: &str = "control.sock";

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
    if let Some(path) = env_path(ENV_CONTROL_SOCKET) {
        return path;
    }

    #[cfg(unix)]
    if let Some(dir) = std::env::var_os("XDG_RUNTIME_DIR") {
        let dir = dir.to_string_lossy();
        if !dir.trim().is_empty() {
            return PathBuf::from(dir.as_ref()).join("orbien").join(SOCKET_FILE);
        }
    }

    state_dir().join("run").join(SOCKET_FILE)
}

fn state_dir() -> PathBuf {
    if let Some(dir) = env_path(ENV_STATE_DIR) {
        return dir;
    }

    if let Some(home) = user_home() {
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

    std::env::temp_dir().join("orbien")
}

fn user_home() -> Option<PathBuf> {
    for key in ["HOME", "USERPROFILE"] {
        if let Some(value) = std::env::var_os(key) {
            if !value.is_empty() {
                return Some(PathBuf::from(value));
            }
        }
    }
    None
}

fn env_path(name: &str) -> Option<PathBuf> {
    let value = std::env::var(name).ok()?;
    let trimmed = value.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(PathBuf::from(trimmed))
    }
}

fn is_stale_socket_error(err: &io::Error) -> bool {
    matches!(
        err.kind(),
        io::ErrorKind::ConnectionRefused
            | io::ErrorKind::NotFound
            | io::ErrorKind::AddrNotAvailable
    )
}

async fn prepare_socket(path: &Path) -> Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("create socket dir {}", parent.display()))?;
    }

    if path.exists() {
        match UnixStream::connect(path).await {
            Ok(stream) => {
                drop(stream);
                bail!(
                    "control socket {} is already in use; another orbien client may be running",
                    path.display()
                );
            }
            Err(e) if is_stale_socket_error(&e) => {
                std::fs::remove_file(path)
                    .with_context(|| format!("remove stale control socket {}", path.display()))?;
            }
            Err(e) => {
                return Err(e).with_context(|| format!("probe control socket {}", path.display()));
            }
        }
    }

    Ok(())
}

fn restrict_socket_permissions(path: &Path) -> Result<()> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o600))?;
    }
    let _ = path;
    Ok(())
}

pub async fn serve(
    socket_path: PathBuf,
    handle: ClientHandle,
    cancel: CancellationToken,
) -> Result<()> {
    prepare_socket(&socket_path).await?;

    let listener = UnixListener::bind(&socket_path)
        .with_context(|| format!("bind control socket {}", socket_path.display()))?;
    restrict_socket_permissions(&socket_path)?;

    tracing::info!(path = %socket_path.display(), "control socket listening");

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
    let socket_path = default_socket_path();
    let stream = UnixStream::connect(&socket_path).await.with_context(|| {
        format!(
            "connect to control socket at {} (start orbien with -c first)",
            socket_path.display()
        )
    })?;
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
