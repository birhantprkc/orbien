use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T: Serialize> {
    pub code: i32,
    pub msg: String,
    pub data: T,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn ok(data: T) -> Self {
        Self {
            code: 200,
            msg: "success".into(),
            data,
        }
    }
}

#[derive(Serialize)]
pub struct Page<T: Serialize> {
    pub total: usize,
    pub page: usize,
    #[serde(rename = "pageSize")]
    pub page_size: usize,
    pub items: Vec<T>,
}

#[derive(Serialize)]
pub struct SystemInfo {
    pub version: String,
    pub config: SystemConfig,
    pub status: SystemStatus,
}

#[derive(Serialize)]
pub struct SystemConfig {
    #[serde(rename = "bindAddr")]
    pub bind_addr: String,
    #[serde(rename = "bindPort")]
    pub bind_port: u16,
    #[serde(rename = "quicBindPort")]
    pub quic_bind_port: u16,
    #[serde(rename = "kcpBindPort")]
    pub kcp_bind_port: u16,
    #[serde(rename = "vhostHTTPPort")]
    pub vhost_http_port: u16,
    #[serde(rename = "vhostHTTPSPort")]
    pub vhost_https_port: u16,
    #[serde(rename = "subDomainHost")]
    pub sub_domain_host: String,
    #[serde(rename = "tcpMux")]
    pub tcp_mux: bool,
    #[serde(rename = "tlsForce")]
    pub tls_force: bool,
    #[serde(rename = "maxPoolCount")]
    pub max_pool_count: i64,
    #[serde(rename = "heartbeatTimeout")]
    pub heartbeat_timeout: i64,
}

#[derive(Serialize)]
pub struct SystemStatus {
    #[serde(rename = "clientCounts")]
    pub client_counts: usize,
    #[serde(rename = "totalClientCounts")]
    pub total_client_counts: usize,
    #[serde(rename = "proxyTypeCount")]
    pub proxy_type_count: std::collections::BTreeMap<String, usize>,
    #[serde(rename = "curConns")]
    pub cur_conns: usize,
    #[serde(rename = "totalTrafficIn")]
    pub total_traffic_in: u64,
    #[serde(rename = "totalTrafficOut")]
    pub total_traffic_out: u64,
}

#[derive(Serialize)]
pub struct ClientInfo {
    #[serde(rename = "runId")]
    pub run_id: String,
    pub user: String,
    pub hostname: String,
    pub os: String,
    pub arch: String,
    #[serde(rename = "clientIP")]
    pub client_ip: String,
    pub version: String,
    #[serde(rename = "proxyCount")]
    pub proxy_count: usize,
    #[serde(rename = "curConns")]
    pub cur_conns: usize,
    #[serde(rename = "connectedSecs")]
    pub connected_secs: u64,
    pub status: String,
}

#[derive(Serialize)]
pub struct ProxyInfo {
    pub name: String,
    #[serde(rename = "type")]
    pub proxy_type: String,
    #[serde(rename = "remoteAddr")]
    pub remote_addr: String,
    #[serde(rename = "localAddr")]
    pub local_addr: String,
    #[serde(rename = "clientId")]
    pub client_id: String,
    pub status: String,
    #[serde(rename = "todayTrafficIn")]
    pub today_traffic_in: u64,
    #[serde(rename = "todayTrafficOut")]
    pub today_traffic_out: u64,
    #[serde(rename = "curConns")]
    pub cur_conns: usize,
    #[serde(rename = "lastStartTime", skip_serializing_if = "Option::is_none")]
    pub last_start_time: Option<String>,
}

#[derive(Serialize)]
pub struct ProxyTrafficPoint {
    pub date: String,
    #[serde(rename = "trafficIn")]
    pub traffic_in: u64,
    #[serde(rename = "trafficOut")]
    pub traffic_out: u64,
}

#[derive(Serialize)]
pub struct ProxyTrafficResp {
    pub name: String,
    pub unit: &'static str,
    pub granularity: &'static str,
    pub history: Vec<ProxyTrafficPoint>,
}
