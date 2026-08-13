use crate::connector::{build_connector, Connector};
use crate::proxy::ProxyManager;
use crate::run_id;
use anyhow::{anyhow, Result};
use orbien_core::auth;
use orbien_core::config::ClientConfig;
use orbien_core::msg::{self, Login, Message, NewProxy, NewWorkConn, Ping};
use orbien_core::transport::DynStream;
use orbien_core::VERSION;
use std::path::Path;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::io::{AsyncWriteExt, ReadHalf, WriteHalf};
use tokio::sync::Mutex;
use tokio::task::JoinSet;
use tokio::time::interval;
use tokio_util::sync::CancellationToken;

#[derive(Debug)]
pub enum SessionEnd {
    Disconnected { run_id: String },
    Kicked { run_id: String, reason: String },
}

type CtrlRead = ReadHalf<DynStream>;
type CtrlWrite = WriteHalf<DynStream>;
type OnProxyRemote = Arc<dyn Fn(String, String) + Send + Sync>;

pub struct Control {
    cfg: ClientConfig,
    run_id: String,
    reader: Mutex<CtrlRead>,
    writer: Mutex<CtrlWrite>,
    proxies: ProxyManager,
    connector: Arc<dyn Connector>,
    cancel: CancellationToken,
    work_tasks: Mutex<JoinSet<()>>,
    on_proxy_remote: OnProxyRemote,
}

impl Control {
    pub async fn start(
        cfg: &ClientConfig,
        previous_run_id: String,
        config_path: &Path,
        parent_cancel: CancellationToken,
        on_connected: impl FnOnce(),
        on_proxy_remote: OnProxyRemote,
    ) -> Result<SessionEnd> {
        let session_cancel = parent_cancel.child_token();
        let connector = build_connector(cfg).await?;
        let mut stream = connector.open().await?;
        tracing::info!(
            endpoint = %cfg.server_endpoint(),
            protocol = %cfg.transport.protocol,
            tcp_mux = cfg.uses_yamux(),
            "control stream opened"
        );

        let timestamp = now_secs();
        let privilege_key = auth::get_auth_key(&cfg.auth.token, timestamp);
        let login = Login {
            version: VERSION.into(),
            hostname: hostname(),
            os: std::env::consts::OS.into(),
            arch: std::env::consts::ARCH.into(),
            user: cfg.user.clone(),
            privilege_key,
            timestamp,
            run_id: previous_run_id,
            pool_count: cfg.transport.pool_count,
        };
        tracing::info!(
            hostname = %login.hostname,
            os = %login.os,
            arch = %login.arch,
            user = %login.user,
            "login identity"
        );

        msg::write_msg(&mut stream, &Message::Login(login)).await?;
        let resp = match msg::read_msg(&mut stream).await? {
            Message::LoginResp(r) => r,
            other => {
                return Err(anyhow!(
                    "expected LoginResp, got type {}",
                    other.type_byte()
                ))
            }
        };

        if !resp.error.is_empty() {
            return Err(anyhow!("login failed: {}", resp.error));
        }

        tracing::info!(run_id = %resp.run_id, "login ok");
        if let Err(e) = run_id::save(config_path, &resp.run_id) {
            tracing::warn!(error = %e, "failed to persist run_id");
        }

        let (reader, writer) = tokio::io::split(stream);
        let ctl = Arc::new(Control {
            cfg: cfg.clone(),
            run_id: resp.run_id.clone(),
            reader: Mutex::new(reader),
            writer: Mutex::new(writer),
            proxies: ProxyManager::from_config(cfg)?,
            connector,
            cancel: session_cancel.clone(),
            work_tasks: Mutex::new(JoinSet::new()),
            on_proxy_remote,
        });

        ctl.register_all_proxies().await?;
        on_connected();

        let hb = Arc::clone(&ctl);
        let hb_cancel = session_cancel.clone();
        let heartbeat = tokio::spawn(async move {
            tokio::select! {
                _ = hb_cancel.cancelled() => {}
                _ = hb.heartbeat_loop() => {}
            }
        });

        let result = ctl.clone().reader_loop().await;
        ctl.shutdown().await;
        heartbeat.abort();
        let _ = heartbeat.await;

        match result {
            Ok(ReaderEnd::Kicked(reason)) => Ok(SessionEnd::Kicked {
                run_id: resp.run_id,
                reason,
            }),
            Ok(ReaderEnd::Closed) => Ok(SessionEnd::Disconnected {
                run_id: resp.run_id,
            }),
            Err(e) => Err(e),
        }
    }

    async fn shutdown(&self) {
        self.cancel.cancel();
        {
            let mut writer = self.writer.lock().await;
            let _ = writer.shutdown().await;
        }
        let mut tasks = self.work_tasks.lock().await;
        tasks.abort_all();
        while tasks.join_next().await.is_some() {}
    }

    async fn register_all_proxies(&self) -> Result<()> {
        for p in &self.cfg.proxies {
            let msg = match p.proxy_type.as_str() {
                "tcp" => Message::NewProxy(new_proxy_base(
                    &p.name,
                    "tcp",
                    p.remote_port as i32,
                    &p.local_ip,
                    p.local_port,
                    &p.transport,
                    |np| {
                        np.custom_domains = Vec::new();
                    },
                )),
                "udp" => Message::NewProxy(new_proxy_base(
                    &p.name,
                    "udp",
                    p.remote_port as i32,
                    &p.local_ip,
                    p.local_port,
                    &p.transport,
                    |_| {},
                )),
                "http" => Message::NewProxy(new_proxy_base(
                    &p.name,
                    "http",
                    0,
                    &p.local_ip,
                    p.local_port,
                    &p.transport,
                    |np| {
                        np.custom_domains = p.custom_domains.clone();
                        np.subdomain = p.subdomain.clone();
                        np.locations = p.locations.clone();
                        np.http_user = p.http_user.clone();
                        np.http_pwd = p.http_password.clone();
                        np.host_header_rewrite = p.host_header_rewrite.clone();
                        np.route_by_http_user = p.route_by_http_user.clone();
                    },
                )),
                "https" => Message::NewProxy(new_proxy_base(
                    &p.name,
                    "https",
                    0,
                    &p.local_ip,
                    p.local_port,
                    &p.transport,
                    |np| {
                        np.custom_domains = p.custom_domains.clone();
                        np.subdomain = p.subdomain.clone();
                    },
                )),
                other => {
                    tracing::warn!(name = %p.name, ty = %other, "skip unsupported proxy type");
                    continue;
                }
            };
            let mut writer = self.writer.lock().await;
            msg::write_msg(&mut *writer, &msg).await?;
            match p.proxy_type.as_str() {
                "tcp" => tracing::info!(
                    name = %p.name,
                    local = %format!("{}:{}", p.local_ip, p.local_port),
                    remote_port = p.remote_port,
                    "sent NewProxy"
                ),
                "udp" => tracing::info!(
                    name = %p.name,
                    local = %format!("{}:{}", p.local_ip, p.local_port),
                    remote_port = p.remote_port,
                    "sent NewProxy udp"
                ),
                "http" => tracing::info!(
                    name = %p.name,
                    local = %format!("{}:{}", p.local_ip, p.local_port),
                    domains = ?p.custom_domains,
                    subdomain = %p.subdomain,
                    "sent NewProxy http"
                ),
                "https" => tracing::info!(
                    name = %p.name,
                    local = %format!("{}:{}", p.local_ip, p.local_port),
                    domains = ?p.custom_domains,
                    subdomain = %p.subdomain,
                    "sent NewProxy https (SNI passthrough)"
                ),
                _ => {}
            }
        }
        Ok(())
    }

    async fn reader_loop(self: Arc<Self>) -> Result<ReaderEnd> {
        loop {
            if self.cancel.is_cancelled() {
                return Ok(ReaderEnd::Closed);
            }

            let msg = tokio::select! {
                _ = self.cancel.cancelled() => {
                    return Ok(ReaderEnd::Closed);
                }
                msg = async {
                    let mut reader = self.reader.lock().await;
                    msg::read_msg(&mut *reader).await
                } => {
                    match msg {
                        Ok(m) => m,
                        Err(_) => return Ok(ReaderEnd::Closed),
                    }
                }
            };

            match msg {
                Message::KickOut(k) => {
                    tracing::warn!(reason = %k.reason, "kicked by server — will exit");
                    return Ok(ReaderEnd::Kicked(k.reason));
                }
                Message::ReqWorkConn(_) => {
                    let ctl = Arc::clone(&self);
                    let cancel = self.cancel.clone();
                    self.work_tasks.lock().await.spawn(async move {
                        tokio::select! {
                            _ = cancel.cancelled() => {}
                            res = ctl.handle_req_work_conn() => {
                                if let Err(e) = res {
                                    tracing::error!(error = %e, "work tunnel failed");
                                }
                            }
                        }
                    });
                }
                Message::NewProxyResp(resp) => {
                    if resp.error.is_empty() {
                        let remote = normalize_remote_addr(
                            &self.cfg.server_addr,
                            &resp.remote_addr,
                        );
                        tracing::info!(
                            name = %resp.proxy_name,
                            remote = %remote,
                            "proxy started"
                        );
                        (self.on_proxy_remote)(resp.proxy_name.clone(), remote);
                    } else {
                        tracing::error!(
                            name = %resp.proxy_name,
                            error = %resp.error,
                            "proxy start failed"
                        );
                    }
                }
                Message::Pong(_) => {
                    tracing::trace!("pong");
                }
                other => {
                    tracing::warn!(ty = other.type_byte(), "ignored message");
                }
            }
        }
    }

    async fn heartbeat_loop(self: Arc<Self>) {
        let secs = self.cfg.transport.heartbeat_interval;
        if secs <= 0 {
            tracing::debug!("app heartbeat disabled (tcpMux / heartbeatInterval<=0)");
            std::future::pending::<()>().await;
            return;
        }
        let mut tick = interval(Duration::from_secs(secs as u64));
        tick.tick().await;
        loop {
            if self.cancel.is_cancelled() {
                break;
            }
            tick.tick().await;
            let timestamp = now_secs();
            let ping = Ping {
                privilege_key: auth::get_auth_key(&self.cfg.auth.token, timestamp),
                timestamp,
            };
            let mut writer = self.writer.lock().await;
            if msg::write_msg(&mut *writer, &Message::Ping(ping))
                .await
                .is_err()
            {
                break;
            }
        }
    }

    async fn handle_req_work_conn(self: Arc<Self>) -> Result<()> {
        let mut work = self.connector.open().await?;

        let timestamp = now_secs();
        msg::write_msg(
            &mut work,
            &Message::NewWorkConn(NewWorkConn {
                run_id: self.run_id.clone(),
                privilege_key: auth::get_auth_key(&self.cfg.auth.token, timestamp),
                timestamp,
            }),
        )
        .await?;

        let start = tokio::select! {
            _ = self.cancel.cancelled() => {
                return Ok(());
            }
            msg = msg::read_msg(&mut work) => {
                match msg? {
                    Message::StartWorkConn(s) => s,
                    other => {
                        return Err(anyhow!("expected StartWorkConn, got {}", other.type_byte()))
                    }
                }
            }
        };

        if !start.error.is_empty() {
            return Err(anyhow!("StartWorkConn error: {}", start.error));
        }

        self.proxies.handle_work_conn(&start, work).await
    }
}

enum ReaderEnd {
    Closed,
    Kicked(String),
}

fn now_secs() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

fn hostname() -> String {
    if let Ok(name) = hostname::get() {
        let s = name.to_string_lossy().trim().to_string();
        if !s.is_empty() {
            return s;
        }
    }

    ["HOSTNAME", "COMPUTERNAME", "HOST"]
        .into_iter()
        .find_map(|k| std::env::var(k).ok())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "unknown".into())
}

fn omit_client_mode(mode: &str) -> String {
    match mode.trim().to_ascii_lowercase().as_str() {
        "" | "client" => String::new(),
        other => other.to_string(),
    }
}

fn normalize_remote_addr(server_addr: &str, remote_addr: &str) -> String {
    let remote = remote_addr.trim();
    if remote.is_empty() {
        return String::new();
    }
    if let Some(port) = remote.strip_prefix(':') {
        let host = server_addr.trim();
        if !host.is_empty() && !port.is_empty() {
            return format!("{host}:{port}");
        }
    }
    remote.to_string()
}

fn new_proxy_base(
    name: &str,
    proxy_type: &str,
    remote_port: i32,
    local_ip: &str,
    local_port: u16,
    transport: &orbien_core::config::ProxyTransportConfig,
    extra: impl FnOnce(&mut NewProxy),
) -> NewProxy {
    let mut np = NewProxy {
        proxy_name: name.into(),
        proxy_type: proxy_type.into(),
        remote_port,
        local_ip: local_ip.into(),
        local_port: i32::from(local_port),
        custom_domains: Vec::new(),
        subdomain: String::new(),
        locations: Vec::new(),
        http_user: String::new(),
        http_pwd: String::new(),
        host_header_rewrite: String::new(),
        headers: Default::default(),
        response_headers: Default::default(),
        route_by_http_user: String::new(),
        bandwidth_limit: transport.bandwidth_limit.clone(),
        bandwidth_limit_mode: omit_client_mode(&transport.bandwidth_limit_mode),
    };
    extra(&mut np);
    np
}
