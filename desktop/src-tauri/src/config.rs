use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ProxyTransportOptions {
    #[serde(default)]
    pub bandwidth_limit: String,
    #[serde(default = "default_bandwidth_mode")]
    pub bandwidth_limit_mode: String,
    #[serde(default)]
    pub proxy_protocol_version: String,
}

fn default_bandwidth_mode() -> String {
    "client".into()
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ProxyPluginConfig {
    #[serde(default, rename = "type", alias = "pluginType")]
    pub plugin_type: String,
    #[serde(default)]
    pub local_addr: String,
    #[serde(default)]
    pub crt_path: String,
    #[serde(default)]
    pub key_path: String,
    #[serde(default)]
    pub host_header_rewrite: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProxyConfig {
    pub name: String,

    #[serde(default = "default_proxy_type")]
    pub proxy_type: String,
    #[serde(default = "default_local_ip")]
    pub local_ip: String,
    #[serde(default)]
    pub local_port: u16,
    #[serde(default)]
    pub remote_port: u16,

    #[serde(default)]
    pub custom_domains: Vec<String>,
    #[serde(default)]
    pub subdomain: String,

    #[serde(default)]
    pub locations: Vec<String>,
    #[serde(default)]
    pub http_user: String,
    #[serde(default)]
    pub http_password: String,
    #[serde(default)]
    pub host_header_rewrite: String,
    #[serde(default)]
    pub route_by_http_user: String,
    #[serde(default)]
    pub transport: ProxyTransportOptions,
    #[serde(default)]
    pub plugin: Option<ProxyPluginConfig>,
}

fn default_proxy_type() -> String {
    "tcp".into()
}

fn default_local_ip() -> String {
    "127.0.0.1".into()
}

fn trim_list(items: Vec<String>) -> Vec<String> {
    items
        .into_iter()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

impl ProxyConfig {
    pub fn normalized(mut self) -> Self {
        self.name = self.name.trim().to_string();
        self.proxy_type = self.proxy_type.trim().to_lowercase();
        if self.proxy_type.is_empty() {
            self.proxy_type = default_proxy_type();
        }
        self.local_ip = self.local_ip.trim().to_string();
        if self.local_ip.is_empty() {
            self.local_ip = default_local_ip();
        }
        self.custom_domains = trim_list(self.custom_domains);
        self.locations = trim_list(self.locations);
        self.subdomain = self.subdomain.trim().to_string();
        self.http_user = self.http_user.trim().to_string();
        self.http_password = self.http_password.trim().to_string();
        self.host_header_rewrite = self.host_header_rewrite.trim().to_string();
        self.route_by_http_user = self.route_by_http_user.trim().to_string();

        self.transport.bandwidth_limit = self.transport.bandwidth_limit.trim().to_string();
        self.transport.bandwidth_limit_mode =
            self.transport.bandwidth_limit_mode.trim().to_lowercase();
        if self.transport.bandwidth_limit_mode.is_empty() {
            self.transport.bandwidth_limit_mode = default_bandwidth_mode();
        }
        self.transport.proxy_protocol_version =
            self.transport.proxy_protocol_version.trim().to_lowercase();

        if let Some(mut pl) = self.plugin.take() {
            pl.plugin_type = pl.plugin_type.trim().to_lowercase();
            pl.local_addr = pl.local_addr.trim().to_string();
            pl.crt_path = pl.crt_path.trim().to_string();
            pl.key_path = pl.key_path.trim().to_string();
            pl.host_header_rewrite = pl.host_header_rewrite.trim().to_string();
            if pl.plugin_type.is_empty() {
                self.plugin = None;
            } else {
                self.plugin = Some(pl);
            }
        }

        match self.proxy_type.as_str() {
            "tcp" | "udp" => {
                self.custom_domains.clear();
                self.subdomain.clear();
                self.locations.clear();
                self.http_user.clear();
                self.http_password.clear();
                self.host_header_rewrite.clear();
                self.route_by_http_user.clear();
                self.plugin = None;
                if self.remote_port == 0 {
                    self.remote_port = 1;
                }
                if self.local_port == 0 {
                    self.local_port = 1;
                }
            }
            "http" => {
                self.remote_port = 0;
                self.plugin = None;
                self.http_user.clear();
                self.http_password.clear();
                self.route_by_http_user.clear();
                if self.local_port == 0 {
                    self.local_port = 80;
                }
            }
            "https" => {
                self.remote_port = 0;
                self.locations.clear();
                self.http_user.clear();
                self.http_password.clear();
                self.route_by_http_user.clear();

                if self.plugin.is_some() {
                    self.local_port = 0;
                    self.host_header_rewrite.clear();
                } else {
                    self.host_header_rewrite.clear();
                    if self.local_port == 0 {
                        self.local_port = 443;
                    }
                }
            }
            _ => {}
        }

        if self.uses_plugin() {
            self.transport.proxy_protocol_version.clear();
        }

        self
    }

    pub fn uses_plugin(&self) -> bool {
        self.plugin
            .as_ref()
            .map(|p| !p.plugin_type.is_empty())
            .unwrap_or(false)
    }

    pub fn local_label(&self) -> String {
        if let Some(pl) = &self.plugin {
            if !pl.plugin_type.is_empty() {
                if pl.local_addr.is_empty() {
                    return format!("plugin:{}", pl.plugin_type);
                }
                return pl.local_addr.clone();
            }
        }
        match self.proxy_type.as_str() {
            "tcp" | "udp" | "http" | "https" => self.local_port.to_string(),
            _ => format!("{}:{}", self.local_ip, self.local_port),
        }
    }

    pub fn remote_hosts(&self) -> Vec<String> {
        let mut hosts = self.custom_domains.clone();
        let sub = self.subdomain.trim();
        if !sub.is_empty() {
            hosts.push(sub.to_string());
        }
        hosts
    }

    pub fn remote_label(&self) -> String {
        match self.proxy_type.as_str() {
            "http" | "https" => {
                let hosts = self.remote_hosts();
                if hosts.is_empty() {
                    "—".into()
                } else {
                    hosts.join(", ")
                }
            }
            "tcp" | "udp" => self.remote_port.to_string(),
            _ => format!(":{}", self.remote_port),
        }
    }

    pub fn copy_address(&self, server_addr: &str) -> String {
        let host = server_addr.trim();
        match self.proxy_type.as_str() {
            "tcp" | "udp" => {
                if host.is_empty() {
                    self.remote_port.to_string()
                } else if host.contains(':') && !host.starts_with('[') {
                    format!("[{host}]:{}", self.remote_port)
                } else {
                    format!("{host}:{}", self.remote_port)
                }
            }
            "http" | "https" => {
                let scheme = if self.proxy_type == "https" {
                    "https"
                } else {
                    "http"
                };
                if self.custom_domains.is_empty() {
                    String::new()
                } else {
                    self.custom_domains
                        .iter()
                        .map(|h| format!("{scheme}://{h}"))
                        .collect::<Vec<_>>()
                        .join("\n")
                }
            }
            _ => String::new(),
        }
    }

    fn to_toml_fragment(&self) -> String {
        let mut out = format!(
            "\n[[proxies]]\nname = \"{name}\"\ntype = \"{ty}\"\n",
            name = escape_toml(&self.name),
            ty = escape_toml(&self.proxy_type),
        );

        let plugin_active = self.uses_plugin();
        if !plugin_active {
            out.push_str(&format!(
                "localIP = \"{lip}\"\nlocalPort = {lport}\n",
                lip = escape_toml(&self.local_ip),
                lport = self.local_port,
            ));
        }

        match self.proxy_type.as_str() {
            "tcp" | "udp" => {
                out.push_str(&format!("remotePort = {}\n", self.remote_port));
            }
            "http" | "https" => {
                if !self.custom_domains.is_empty() {
                    out.push_str(&format!(
                        "customDomains = {}\n",
                        toml_string_array(&self.custom_domains)
                    ));
                }
                if !self.subdomain.is_empty() {
                    out.push_str(&format!(
                        "subdomain = \"{}\"\n",
                        escape_toml(&self.subdomain)
                    ));
                }
                if self.proxy_type == "http" {
                    if !self.locations.is_empty() {
                        out.push_str(&format!(
                            "locations = {}\n",
                            toml_string_array(&self.locations)
                        ));
                    }

                    if !self.host_header_rewrite.is_empty() {
                        out.push_str(&format!(
                            "hostHeaderRewrite = \"{}\"\n",
                            escape_toml(&self.host_header_rewrite)
                        ));
                    }
                }
            }
            _ => {}
        }

        if !self.transport.bandwidth_limit.is_empty() {
            out.push_str(&format!(
                "transport.bandwidthLimit = \"{}\"\n",
                escape_toml(&self.transport.bandwidth_limit)
            ));
            out.push_str(&format!(
                "transport.bandwidthLimitMode = \"{}\"\n",
                escape_toml(&self.transport.bandwidth_limit_mode)
            ));
        }
        if !self.transport.proxy_protocol_version.is_empty() {
            out.push_str(&format!(
                "transport.proxyProtocolVersion = \"{}\"\n",
                escape_toml(&self.transport.proxy_protocol_version)
            ));
        }

        if let Some(pl) = &self.plugin {
            if !pl.plugin_type.is_empty() {
                out.push_str(&format!(
                    "\n[proxies.plugin]\ntype = \"{ty}\"\nlocalAddr = \"{addr}\"\ncrtPath = \"{crt}\"\nkeyPath = \"{key}\"\nhostHeaderRewrite = \"{hh}\"\n",
                    ty = escape_toml(&pl.plugin_type),
                    addr = escape_toml(&pl.local_addr),
                    crt = escape_toml(&pl.crt_path),
                    key = escape_toml(&pl.key_path),
                    hh = escape_toml(&pl.host_header_rewrite),
                ));
            }
        }

        out
    }
}

fn toml_string_array(items: &[String]) -> String {
    let inner = items
        .iter()
        .map(|s| format!("\"{}\"", escape_toml(s)))
        .collect::<Vec<_>>()
        .join(", ");
    format!("[{inner}]")
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TlsConfig {
    #[serde(default = "default_true")]
    pub enable: bool,
    #[serde(default)]
    pub cert_file: String,
    #[serde(default)]
    pub key_file: String,
    #[serde(default)]
    pub trusted_ca_file: String,
    #[serde(default)]
    pub server_name: String,
    #[serde(default = "default_true")]
    pub disable_custom_tls_first_byte: bool,
}

fn default_true() -> bool {
    true
}

impl Default for TlsConfig {
    fn default() -> Self {
        Self {
            enable: true,
            cert_file: String::new(),
            key_file: String::new(),
            trusted_ca_file: String::new(),
            server_name: String::new(),
            disable_custom_tls_first_byte: true,
        }
    }
}

impl TlsConfig {
    fn normalized(mut self) -> Self {
        self.cert_file = self.cert_file.trim().to_string();
        self.key_file = self.key_file.trim().to_string();
        self.trusted_ca_file = self.trusted_ca_file.trim().to_string();
        self.server_name = self.server_name.trim().to_string();
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuicConfig {
    #[serde(default = "default_quic_keepalive")]
    pub keepalive_period: u64,
    #[serde(default = "default_quic_idle")]
    pub max_idle_timeout: u64,
    #[serde(default = "default_quic_streams")]
    pub max_incoming_streams: u32,
}

fn default_quic_keepalive() -> u64 {
    10
}
fn default_quic_idle() -> u64 {
    30
}
fn default_quic_streams() -> u32 {
    100_000
}

impl Default for QuicConfig {
    fn default() -> Self {
        Self {
            keepalive_period: default_quic_keepalive(),
            max_idle_timeout: default_quic_idle(),
            max_incoming_streams: default_quic_streams(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientConfig {
    pub server_addr: String,
    pub server_port: u16,

    #[serde(default)]
    pub user: String,
    pub token: String,

    #[serde(default = "default_udp_packet_size")]
    pub udp_packet_size: u32,
    pub protocol: String,
    #[serde(default = "default_pool_count")]
    pub pool_count: i32,
    pub tcp_mux: bool,
    #[serde(default = "default_tcp_mux_keepalive")]
    pub tcp_mux_keepalive_interval: i64,
    #[serde(default = "default_heartbeat_interval")]
    pub heartbeat_interval: i64,
    #[serde(default = "default_heartbeat_timeout")]
    pub heartbeat_timeout: i64,
    #[serde(default)]
    pub tls: TlsConfig,
    #[serde(default)]
    pub quic: QuicConfig,

    pub orbien_path: String,

    #[serde(default)]
    pub proxies: Vec<ProxyConfig>,
}

fn default_udp_packet_size() -> u32 {
    1500
}
fn default_pool_count() -> i32 {
    1
}
fn default_tcp_mux_keepalive() -> i64 {
    30
}
fn default_heartbeat_interval() -> i64 {
    -1
}
fn default_heartbeat_timeout() -> i64 {
    -1
}

impl ClientConfig {
    pub fn normalized(mut self) -> Self {
        self.server_addr = self.server_addr.trim().to_string();
        self.user = self.user.trim().to_string();
        self.token = self.token.trim().to_string();
        self.protocol = self.protocol.trim().to_lowercase();
        if self.protocol.is_empty() {
            self.protocol = "tcp".into();
        }
        if self.protocol == "ws" {
            self.protocol = "websocket".into();
        }
        self.orbien_path.clear();
        if self.server_port == 0 {
            self.server_port = 9527;
        }
        if self.udp_packet_size == 0 {
            self.udp_packet_size = 1500;
        }
        if self.pool_count < 0 {
            self.pool_count = 1;
        }
        if self.tcp_mux_keepalive_interval <= 0 {
            self.tcp_mux_keepalive_interval = 30;
        }

        if !self.tcp_mux {
            if self.heartbeat_interval < 0 {
                self.heartbeat_interval = 30;
            }
            if self.heartbeat_timeout < 0 {
                self.heartbeat_timeout = 90;
            }
        }
        if self.quic.keepalive_period == 0 {
            self.quic.keepalive_period = 10;
        }
        if self.quic.max_idle_timeout == 0 {
            self.quic.max_idle_timeout = 30;
        }
        if self.quic.max_incoming_streams == 0 {
            self.quic.max_incoming_streams = 100_000;
        }
        self.tls = self.tls.normalized();
        self.proxies = self
            .proxies
            .into_iter()
            .map(ProxyConfig::normalized)
            .filter(|p| !p.name.is_empty())
            .collect();
        self
    }

    pub fn to_orbien_toml(&self) -> String {
        let mut out = format!(
            r#"# Generated by Orbien Desktop — do not edit while app is running
serverAddr = "{addr}"
serverPort = {port}
user = "{user}"
udpPacketSize = {udp}

[auth]
method = "token"
token = "{token}"

[transport]
protocol = "{proto}"
tcpMux = {mux}
tcpMuxKeepaliveInterval = {mux_ka}
poolCount = {pool}
heartbeatInterval = {hb_i}
heartbeatTimeout = {hb_t}

[transport.tls]
enable = {tls_en}
certFile = "{tls_cert}"
keyFile = "{tls_key}"
trustedCaFile = "{tls_ca}"
serverName = "{tls_sni}"
disableCustomTLSFirstByte = {tls_first}

[transport.quic]
keepalivePeriod = {q_ka}
maxIdleTimeout = {q_idle}
maxIncomingStreams = {q_streams}
"#,
            addr = escape_toml(&self.server_addr),
            port = self.server_port,
            user = escape_toml(&self.user),
            udp = self.udp_packet_size,
            token = escape_toml(&self.token),
            proto = escape_toml(&self.protocol),
            mux = self.tcp_mux,
            mux_ka = self.tcp_mux_keepalive_interval,
            pool = self.pool_count,
            hb_i = self.heartbeat_interval,
            hb_t = self.heartbeat_timeout,
            tls_en = self.tls.enable,
            tls_cert = escape_toml(&self.tls.cert_file),
            tls_key = escape_toml(&self.tls.key_file),
            tls_ca = escape_toml(&self.tls.trusted_ca_file),
            tls_sni = escape_toml(&self.tls.server_name),
            tls_first = self.tls.disable_custom_tls_first_byte,
            q_ka = self.quic.keepalive_period,
            q_idle = self.quic.max_idle_timeout,
            q_streams = self.quic.max_incoming_streams,
        );

        for p in &self.proxies {
            out.push_str(&p.to_toml_fragment());
        }

        out
    }
}

fn escape_toml(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

pub fn default_config() -> ClientConfig {
    ClientConfig {
        server_addr: "127.0.0.1".into(),
        server_port: 9527,
        user: String::new(),
        token: String::new(),
        udp_packet_size: 1500,
        protocol: "tcp".into(),
        pool_count: 1,
        tcp_mux: true,
        tcp_mux_keepalive_interval: 30,
        heartbeat_interval: -1,
        heartbeat_timeout: -1,
        tls: TlsConfig::default(),
        quic: QuicConfig::default(),
        orbien_path: String::new(),
        proxies: Vec::new(),
    }
    .normalized()
}

fn config_file(app: &AppHandle) -> Result<PathBuf, String> {
    let dir = app
        .path()
        .app_config_dir()
        .map_err(|e| format!("app config dir: {e}"))?;
    fs::create_dir_all(&dir).map_err(|e| format!("mkdir config: {e}"))?;
    Ok(dir.join("client.json"))
}

fn runtime_toml_path(app: &AppHandle) -> Result<PathBuf, String> {
    let dir = app
        .path()
        .app_config_dir()
        .map_err(|e| format!("app config dir: {e}"))?;
    fs::create_dir_all(&dir).map_err(|e| format!("mkdir config: {e}"))?;
    Ok(dir.join("orbien.runtime.toml"))
}

pub fn load_config(app: &AppHandle) -> Result<ClientConfig, String> {
    let path = config_file(app)?;
    if !path.exists() {
        let cfg = default_config();
        save_config(app, &cfg)?;
        return Ok(cfg);
    }
    let raw = fs::read_to_string(&path).map_err(|e| format!("read {}: {e}", path.display()))?;
    let cfg: ClientConfig =
        serde_json::from_str(&raw).map_err(|e| format!("parse {}: {e}", path.display()))?;
    Ok(cfg.normalized())
}

pub fn save_config(app: &AppHandle, cfg: &ClientConfig) -> Result<(), String> {
    let path = config_file(app)?;
    let raw = serde_json::to_string_pretty(cfg).map_err(|e| e.to_string())?;
    fs::write(&path, raw).map_err(|e| format!("write {}: {e}", path.display()))
}

pub fn write_runtime_toml(app: &AppHandle, cfg: &ClientConfig) -> Result<PathBuf, String> {
    let path = runtime_toml_path(app)?;
    fs::write(&path, cfg.to_orbien_toml()).map_err(|e| format!("write {}: {e}", path.display()))?;
    Ok(path)
}
