mod config;
mod pick_file;
mod process;

use config::{
    default_config, load_config, save_config, write_runtime_toml, ClientConfig, ProxyConfig,
};
use process::{resolve_orbien, spawn_orbien, stop_child};
use serde::Serialize;
use std::collections::VecDeque;
use std::process::Child;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use tauri::{AppHandle, Manager, State};

const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

const RESTART_GAP: Duration = Duration::from_millis(400);
const MAX_LOG_LINES: usize = 800;
const MAX_LOG_LINE_CHARS: usize = 4_096;

pub(crate) struct SessionInner {
    running: bool,
    started_at: Option<Instant>,
    logs: VecDeque<String>,
    logs_rev: u64,
    child: Option<Child>,
    config: ClientConfig,
}

pub struct AppState {
    inner: Arc<Mutex<SessionInner>>,
}

impl AppState {
    fn new(config: ClientConfig) -> Self {
        let mut session = SessionInner {
            running: false,
            started_at: None,
            logs: VecDeque::new(),
            logs_rev: 0,
            child: None,
            config,
        };
        push_log(
            &mut session,
            format!("[info] Orbien Desktop {APP_VERSION} ready"),
        );
        Self {
            inner: Arc::new(Mutex::new(session)),
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ClientStatus {
    running: bool,
    running_secs: u64,
    version: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct RuntimeStats {
    cpu_percent: f64,
    memory_mb: u64,
    version: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ProxyItem {
    name: String,
    proxy_type: String,
    local: String,
    remote: String,
    copy_value: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SaveProxiesResult {
    proxies: Vec<ProxyItem>,
    restarted: bool,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SaveConfigResult {
    config: ClientConfig,
    restarted: bool,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct LogsSnapshot {
    rev: u64,
    lines: Option<Vec<String>>,
}

fn push_log(session: &mut SessionInner, line: impl Into<String>) {
    let mut line = strip_ansi(&line.into());
    if line.trim().is_empty() {
        return;
    }
    if line.len() > MAX_LOG_LINE_CHARS {
        let mut end = MAX_LOG_LINE_CHARS;
        while end > 0 && !line.is_char_boundary(end) {
            end -= 1;
        }
        line.truncate(end);
        line.push('…');
    }
    session.logs.push_back(line);
    while session.logs.len() > MAX_LOG_LINES {
        session.logs.pop_front();
    }
    session.logs_rev = session.logs_rev.wrapping_add(1);
}

fn strip_ansi(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut chars = input.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\u{1b}' {
            if chars.peek() == Some(&'[') {
                chars.next();
                for ch in chars.by_ref() {
                    if ch.is_ascii_alphabetic() {
                        break;
                    }
                }
                continue;
            }
            continue;
        }

        if c.is_control() && c != '\t' {
            continue;
        }
        out.push(c);
    }
    out
}

fn status_from(session: &SessionInner) -> ClientStatus {
    ClientStatus {
        running: session.running,
        running_secs: session
            .started_at
            .map(|t| t.elapsed().as_secs())
            .unwrap_or(0),
        version: APP_VERSION.into(),
    }
}

fn proxy_items(cfg: &ClientConfig) -> Vec<ProxyItem> {
    cfg.proxies
        .iter()
        .map(|p| ProxyItem {
            name: p.name.clone(),
            proxy_type: p.proxy_type.clone(),
            local: p.local_label(),
            remote: p.remote_label(),
            copy_value: p.copy_address(&cfg.server_addr),
        })
        .collect()
}

fn reap_if_exited(session: &mut SessionInner) {
    if !session.running {
        return;
    }
    let Some(child) = session.child.as_mut() else {
        return;
    };
    match child.try_wait() {
        Ok(Some(status)) => {
            let secs = session
                .started_at
                .map(|t| t.elapsed())
                .unwrap_or(Duration::ZERO)
                .as_secs();
            session.child = None;
            session.running = false;
            session.started_at = None;
            push_log(
                session,
                format!("[warn] orbien exited after {secs}s ({status})"),
            );
        }
        Ok(None) => {}
        Err(e) => {
            push_log(session, format!("[error] wait orbien failed: {e}"));
            session.child = None;
            session.running = false;
            session.started_at = None;
        }
    }
}

fn stop_session(session: &mut SessionInner, reason: &str) {
    let secs = session
        .started_at
        .map(|t| t.elapsed())
        .unwrap_or(Duration::ZERO)
        .as_secs();

    if let Some(child) = session.child.take() {
        match stop_child(child) {
            Ok(()) => push_log(
                session,
                format!("[info] orbien stopped after {secs}s ({reason})"),
            ),
            Err(e) => push_log(session, format!("[error] stop orbien: {e}")),
        }
    } else if session.running {
        push_log(session, "[info] cleared stale running flag");
    }

    session.running = false;
    session.started_at = None;
}

fn start_session(
    app: &AppHandle,
    logs: Arc<Mutex<SessionInner>>,
    session: &mut SessionInner,
) -> Result<(), String> {
    let cfg = session.config.clone();
    let toml_path = match write_runtime_toml(app, &cfg) {
        Ok(p) => p,
        Err(e) => {
            push_log(session, format!("[error] write runtime config failed: {e}"));
            return Err(e);
        }
    };
    let bin = match resolve_orbien(&cfg.orbien_path) {
        Ok(p) => p,
        Err(e) => {
            push_log(session, format!("[error] {e}"));
            return Err(e);
        }
    };
    push_log(
        session,
        format!(
            "[info] starting {} -c {}",
            bin.display(),
            toml_path.display()
        ),
    );

    let child = match spawn_orbien(&bin, &toml_path, logs) {
        Ok(c) => c,
        Err(e) => {
            push_log(session, format!("[error] {e}"));
            return Err(e);
        }
    };

    session.child = Some(child);
    session.running = true;
    session.started_at = Some(Instant::now());
    push_log(
        session,
        format!(
            "[info] orbien started → {}:{} ({}) with {} proxy(ies)",
            cfg.server_addr,
            cfg.server_port,
            cfg.protocol,
            cfg.proxies.len()
        ),
    );
    Ok(())
}

fn restart_session(
    app: &AppHandle,
    logs: Arc<Mutex<SessionInner>>,
    session: &mut SessionInner,
    reason: &str,
) -> Result<(), String> {
    push_log(
        session,
        format!("[info] applying changes via restart ({reason})"),
    );
    stop_session(session, reason);

    thread::sleep(RESTART_GAP);
    start_session(app, logs, session)
}

#[tauri::command]
fn get_status(state: State<'_, AppState>) -> ClientStatus {
    let mut s = state.inner.lock().expect("session lock");
    reap_if_exited(&mut s);
    status_from(&s)
}

#[tauri::command]
fn get_config(state: State<'_, AppState>) -> ClientConfig {
    state.inner.lock().expect("session lock").config.clone()
}

#[tauri::command]
fn save_client_config(
    app: AppHandle,
    state: State<'_, AppState>,
    config: ClientConfig,
) -> Result<SaveConfigResult, String> {
    let cfg = config.normalized();
    save_config(&app, &cfg)?;

    let mut s = state.inner.lock().map_err(|e| e.to_string())?;
    reap_if_exited(&mut s);
    let was_running = s.running;
    s.config = cfg.clone();
    push_log(&mut s, "[info] config saved");

    let restarted = if was_running {
        restart_session(&app, Arc::clone(&state.inner), &mut s, "config updated")?;
        true
    } else {
        false
    };

    Ok(SaveConfigResult {
        config: cfg,
        restarted,
    })
}

#[tauri::command]
fn start_client(app: AppHandle, state: State<'_, AppState>) -> Result<ClientStatus, String> {
    let mut s = state.inner.lock().map_err(|e| e.to_string())?;
    reap_if_exited(&mut s);
    if s.running {
        return Err("client already running".into());
    }
    start_session(&app, Arc::clone(&state.inner), &mut s)?;
    Ok(status_from(&s))
}

#[tauri::command]
fn stop_client(state: State<'_, AppState>) -> Result<ClientStatus, String> {
    let mut s = state.inner.lock().map_err(|e| e.to_string())?;
    if !s.running && s.child.is_none() {
        return Err("client is not running".into());
    }
    stop_session(&mut s, "user stop");
    Ok(status_from(&s))
}

#[tauri::command]
fn get_logs(state: State<'_, AppState>, since_rev: u64) -> LogsSnapshot {
    let s = state.inner.lock().expect("session lock");
    if since_rev == s.logs_rev && since_rev != 0 {
        return LogsSnapshot {
            rev: s.logs_rev,
            lines: None,
        };
    }
    LogsSnapshot {
        rev: s.logs_rev,
        lines: Some(s.logs.iter().cloned().collect()),
    }
}

#[tauri::command]
fn clear_logs(state: State<'_, AppState>) {
    let mut s = state.inner.lock().expect("session lock");
    s.logs.clear();
    push_log(&mut s, "[info] logs cleared");
}

#[tauri::command]
fn get_runtime_stats() -> RuntimeStats {
    RuntimeStats {
        cpu_percent: 0.0,
        memory_mb: 0,
        version: APP_VERSION.into(),
    }
}

#[tauri::command]
fn list_proxies(state: State<'_, AppState>) -> Vec<ProxyItem> {
    let cfg = state.inner.lock().expect("session lock").config.clone();
    proxy_items(&cfg)
}

#[tauri::command]
fn save_proxies(
    app: AppHandle,
    state: State<'_, AppState>,
    proxies: Vec<ProxyConfig>,
) -> Result<SaveProxiesResult, String> {
    let next: Vec<ProxyConfig> = proxies
        .into_iter()
        .map(ProxyConfig::normalized)
        .filter(|p| !p.name.is_empty())
        .collect();

    let mut s = state.inner.lock().map_err(|e| e.to_string())?;
    reap_if_exited(&mut s);

    let changed = s.config.proxies != next;
    let was_running = s.running;
    s.config.proxies = next;
    save_config(&app, &s.config)?;
    let proxy_count = s.config.proxies.len();
    push_log(
        &mut s,
        format!(
            "[info] proxies saved ({} item(s){})",
            proxy_count,
            if changed { ", changed" } else { ", unchanged" }
        ),
    );

    let restarted = if was_running && changed {
        restart_session(&app, Arc::clone(&state.inner), &mut s, "proxies updated")?;
        true
    } else {
        false
    };

    Ok(SaveProxiesResult {
        proxies: proxy_items(&s.config),
        restarted,
    })
}

pub(crate) fn append_log_line(state: &Arc<Mutex<SessionInner>>, line: String) {
    if let Ok(mut s) = state.lock() {
        push_log(&mut s, line);
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let cfg = load_config(app.handle()).unwrap_or_else(|e| {
                eprintln!("load config failed: {e}; using defaults");
                default_config()
            });
            app.manage(AppState::new(cfg));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_status,
            get_config,
            save_client_config,
            start_client,
            stop_client,
            get_logs,
            clear_logs,
            get_runtime_stats,
            list_proxies,
            save_proxies,
            pick_file::pick_file,
        ])
        .run(tauri::generate_context!())
        .expect("error while running Orbien Desktop");
}
