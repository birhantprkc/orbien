use super::{HttpProxy, HttpsProxy, TcpProxy, UdpProxy};
use std::collections::HashMap;

pub enum RegisteredProxy {
    Tcp(TcpProxy),
    Http(HttpProxy),
    Https(HttpsProxy),
    Udp(UdpProxy),
}

impl RegisteredProxy {
    pub fn proxy_type(&self) -> &'static str {
        match self {
            Self::Tcp(_) => "tcp",
            Self::Http(_) => "http",
            Self::Https(_) => "https",
            Self::Udp(_) => "udp",
        }
    }

    pub async fn close(&self) {
        match self {
            Self::Tcp(p) => p.close().await,
            Self::Http(p) => p.close().await,
            Self::Https(p) => p.close().await,
            Self::Udp(p) => p.close().await,
        }
    }
}

struct ProxyEntry {
    proxy: RegisteredProxy,
    local_addr: String,
}

pub struct ProxyManager {
    proxies: HashMap<String, ProxyEntry>,
}

impl ProxyManager {
    pub fn new() -> Self {
        Self {
            proxies: HashMap::new(),
        }
    }

    pub async fn insert(
        &mut self,
        name: String,
        proxy: RegisteredProxy,
        local_addr: String,
    ) -> Option<&'static str> {
        let entry = ProxyEntry { proxy, local_addr };
        if let Some(old) = self.proxies.insert(name, entry) {
            let ty = old.proxy.proxy_type();
            old.proxy.close().await;
            Some(ty)
        } else {
            None
        }
    }

    pub async fn remove(&mut self, name: &str) -> Option<&'static str> {
        if let Some(entry) = self.proxies.remove(name) {
            let ty = entry.proxy.proxy_type();
            entry.proxy.close().await;
            Some(ty)
        } else {
            None
        }
    }

    pub async fn close_all(&mut self) -> Vec<(String, &'static str)> {
        let mut closed = Vec::with_capacity(self.proxies.len());
        for (name, entry) in self.proxies.drain() {
            closed.push((name, entry.proxy.proxy_type()));
            entry.proxy.close().await;
        }
        closed
    }

    pub fn summaries(&self) -> Vec<ProxySummary> {
        self.proxies
            .iter()
            .map(|(name, entry)| {
                let (proxy_type, remote_addr) = match &entry.proxy {
                    RegisteredProxy::Tcp(t) => ("tcp".into(), format!(":{}", t.remote_port)),
                    RegisteredProxy::Http(h) => ("http".into(), h.domains.join(",")),
                    RegisteredProxy::Https(h) => ("https".into(), h.domains.join(",")),
                    RegisteredProxy::Udp(u) => ("udp".into(), format!(":{}", u.remote_port)),
                };
                ProxySummary {
                    name: name.clone(),
                    proxy_type,
                    remote_addr,
                    local_addr: entry.local_addr.clone(),
                    status: "online".into(),
                }
            })
            .collect()
    }

    pub fn len(&self) -> usize {
        self.proxies.len()
    }
}

pub fn format_local_addr(ip: &str, port: i32) -> String {
    let ip = ip.trim();
    if ip.is_empty() && port <= 0 {
        return String::new();
    }
    if ip.is_empty() {
        return format!(":{port}");
    }
    if port <= 0 {
        return ip.to_string();
    }
    if ip.contains(':') && !ip.starts_with('[') {
        format!("[{ip}]:{port}")
    } else {
        format!("{ip}:{port}")
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct ProxySummary {
    pub name: String,
    #[serde(rename = "type")]
    pub proxy_type: String,
    #[serde(rename = "remoteAddr")]
    pub remote_addr: String,
    #[serde(rename = "localAddr")]
    pub local_addr: String,
    pub status: String,
}
