use anyhow::Result;
use orbien_core::tls::PrefixedStream;
use std::net::SocketAddr;
use tokio::net::TcpStream;

#[derive(Debug, Clone, Default)]
pub struct AccessPolicy;

impl AccessPolicy {
    pub fn from_server_config(_cfg: &orbien_core::config::ServerConfig) -> Result<Self> {
        Ok(Self)
    }
}

pub struct IngressConn {
    pub stream: PrefixedStream<TcpStream>,
    pub peer: SocketAddr,
    pub source: SocketAddr,
    pub local: Option<SocketAddr>,
}

pub async fn prepare_ingress(
    stream: TcpStream,
    peer: SocketAddr,
    _policy: &AccessPolicy,
) -> Result<IngressConn> {
    let local = stream.local_addr().ok();
    Ok(IngressConn {
        stream: PrefixedStream::new(Vec::new(), stream),
        peer,
        source: peer,
        local,
    })
}
