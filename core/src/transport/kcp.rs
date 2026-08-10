use super::stream::{boxed_stream, DynStream};
use anyhow::{Context, Result};
use kcp_tokio::config::NodeDelayConfig;
use kcp_tokio::{KcpConfig, KcpListener, KcpStream};
use std::net::SocketAddr;
use std::pin::Pin;
use std::task::{Context as TaskContext, Poll};
use std::time::Duration;
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};

pub fn default_kcp_config() -> KcpConfig {
    KcpConfig::new()
        .nodelay_config(NodeDelayConfig::custom(true, 20, 2, true))
        .stream_mode(true)
        .mtu(1350)
        .window_size(2048, 2048)
        .max_retries(64)
        .connect_timeout(Duration::from_secs(10))
        .keep_alive(Some(Duration::from_secs(20)))
        .socket_buffer_size(4 * 1024 * 1024)
}

pub async fn dial_kcp(addr: SocketAddr) -> Result<DynStream> {
    let stream = KcpStream::connect(addr, default_kcp_config())
        .await
        .with_context(|| format!("kcp dial {addr}"))?;
    Ok(KcpByteStream::new(stream).boxed())
}

pub async fn bind_kcp_listener(addr: SocketAddr) -> Result<KcpListener> {
    KcpListener::bind(addr, default_kcp_config())
        .await
        .with_context(|| format!("kcp listen {addr}"))
}

pub async fn accept_kcp(listener: &mut KcpListener) -> Result<(DynStream, SocketAddr)> {
    let (stream, peer) = listener.accept().await.context("kcp accept")?;
    Ok((KcpByteStream::new(stream).boxed(), peer))
}

pub struct KcpByteStream {
    inner: KcpStream,
}

impl KcpByteStream {
    pub fn new(inner: KcpStream) -> Self {
        Self { inner }
    }

    pub fn boxed(self) -> DynStream {
        boxed_stream(self)
    }
}

impl AsyncRead for KcpByteStream {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut TaskContext<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        Pin::new(&mut self.inner).poll_read(cx, buf)
    }
}

impl AsyncWrite for KcpByteStream {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut TaskContext<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, std::io::Error>> {
        Pin::new(&mut self.inner).poll_write(cx, buf)
    }

    fn poll_flush(
        mut self: Pin<&mut Self>,
        cx: &mut TaskContext<'_>,
    ) -> Poll<Result<(), std::io::Error>> {
        Pin::new(&mut self.inner).poll_flush(cx)
    }

    fn poll_shutdown(
        mut self: Pin<&mut Self>,
        cx: &mut TaskContext<'_>,
    ) -> Poll<Result<(), std::io::Error>> {
        Pin::new(&mut self.inner).poll_flush(cx)
    }
}
