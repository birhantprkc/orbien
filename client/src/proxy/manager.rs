use super::udp::run_udp_session;
use crate::plugin::{self, ConnectionInfo, Plugin, PluginContext};
use anyhow::{anyhow, Result};
use orbien_core::config::{ClientConfig, ProxyConfig};
use orbien_core::io;
use orbien_core::limit::{self, maybe_limit, BandwidthLimitMode, BandwidthLimiter};
use orbien_core::msg::StartWorkConn;
use orbien_core::net::{
    addrs_from_start_work, build_proxy_protocol_header, parse_proxy_protocol_version,
};
use orbien_core::transport::DynStream;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio::sync::{oneshot, Mutex};

struct ProxyEntry {
    cfg: ProxyConfig,
    limiter: Option<Arc<BandwidthLimiter>>,
    plugin: Option<Arc<dyn Plugin>>,
    proxy_protocol: Option<&'static str>,
    udp_cancel: Mutex<Option<oneshot::Sender<()>>>,
}

pub struct ProxyManager {
    by_name: HashMap<String, ProxyEntry>,
    udp_packet_size: usize,
}

impl ProxyManager {
    pub fn from_config(cfg: &ClientConfig) -> Result<Self> {
        let mut by_name = HashMap::new();
        for p in &cfg.proxies {
            let limiter = limit::limiter_if_mode(
                &p.transport.bandwidth_limit,
                &p.transport.bandwidth_limit_mode,
                BandwidthLimitMode::Client,
            )
            .unwrap_or_else(|e| {
                tracing::warn!(
                    proxy = %p.name,
                    error = %e,
                    "invalid bandwidthLimit; ignoring"
                );
                None
            });
            if let Some(ref l) = limiter {
                tracing::info!(
                    proxy = %p.name,
                    bytes_per_sec = l.bytes_per_sec(),
                    mode = "client",
                    "bandwidth limit enabled"
                );
            }

            let plugin = if let Some(ref pc) = p.plugin {
                if pc.plugin_type.is_empty() {
                    None
                } else {
                    let cn = p.custom_domains.first().cloned().unwrap_or_else(|| {
                        if p.subdomain.is_empty() {
                            "localhost".into()
                        } else {
                            p.subdomain.clone()
                        }
                    });
                    let ctx = PluginContext {
                        name: p.name.clone(),
                        cert_common_name: cn,
                    };
                    Some(plugin::create(ctx, pc)?)
                }
            } else {
                None
            };

            let proxy_protocol = parse_proxy_protocol_version(&p.transport.proxy_protocol_version)?;
            if let Some(ver) = proxy_protocol {
                tracing::info!(
                    proxy = %p.name,
                    version = ver,
                    "proxy protocol enabled (client writes PP to local)"
                );
            }

            by_name.insert(
                p.name.clone(),
                ProxyEntry {
                    cfg: p.clone(),
                    limiter,
                    plugin,
                    proxy_protocol,
                    udp_cancel: Mutex::new(None),
                },
            );
        }
        Ok(Self {
            by_name,
            udp_packet_size: cfg.udp_packet_size.max(512),
        })
    }

    pub async fn handle_work_conn(&self, start: &StartWorkConn, work: DynStream) -> Result<()> {
        let entry = self
            .by_name
            .get(&start.proxy_name)
            .ok_or_else(|| anyhow!("unknown proxy: {}", start.proxy_name))?;

        match entry.cfg.proxy_type.as_str() {
            "udp" => self.handle_udp(entry, work).await,
            "tcp" | "http" | "https" => self.handle_stream_proxy(entry, start, work).await,
            other => Err(anyhow!(
                "unsupported proxy type on work conn: {} ({})",
                other,
                entry.cfg.name
            )),
        }
    }

    async fn handle_stream_proxy(
        &self,
        entry: &ProxyEntry,
        start: &StartWorkConn,
        work: DynStream,
    ) -> Result<()> {
        let work = maybe_limit(work, entry.limiter.clone());

        if let Some(ref plugin) = entry.plugin {
            tracing::debug!(
                proxy = %entry.cfg.name,
                plugin = plugin.name(),
                "handle by plugin"
            );
            return plugin
                .handle(ConnectionInfo {
                    stream: work,
                    src_addr: start.src_addr.clone(),
                    src_port: start.src_port,
                    dst_addr: start.dst_addr.clone(),
                    dst_port: start.dst_port,
                })
                .await;
        }

        let local_addr = format!("{}:{}", entry.cfg.local_ip, entry.cfg.local_port);
        let mut local = TcpStream::connect(&local_addr).await.map_err(|e| {
            anyhow!(
                "dial local {} for proxy {}: {}",
                local_addr,
                entry.cfg.name,
                e
            )
        })?;

        if let Some(ver) = entry.proxy_protocol {
            if let Some((src, dst)) = addrs_from_start_work(
                &start.src_addr,
                start.src_port,
                &start.dst_addr,
                start.dst_port,
                entry.cfg.local_port,
            ) {
                let hdr = build_proxy_protocol_header(src, dst, ver)?;
                local.write_all(&hdr).await?;
                tracing::debug!(
                    proxy = %entry.cfg.name,
                    version = ver,
                    %src,
                    %dst,
                    "wrote proxy protocol header to local"
                );
            } else {
                tracing::debug!(
                    proxy = %entry.cfg.name,
                    "proxy protocol configured but StartWorkConn src empty; skip"
                );
            }
        }

        tracing::debug!(
            proxy = %entry.cfg.name,
            %local_addr,
            limited = entry.limiter.is_some(),
            "joining work <-> local"
        );
        let _ = io::join(work, local).await;
        Ok(())
    }

    async fn handle_udp(&self, entry: &ProxyEntry, work: DynStream) -> Result<()> {
        let local_addr: std::net::SocketAddr =
            format!("{}:{}", entry.cfg.local_ip, entry.cfg.local_port)
                .parse()
                .map_err(|e| anyhow!("invalid local udp addr: {e}"))?;

        let (cancel_tx, cancel_rx) = oneshot::channel();
        {
            let mut slot = entry.udp_cancel.lock().await;
            if let Some(old) = slot.take() {
                let _ = old.send(());
            }
            *slot = Some(cancel_tx);
        }

        tracing::info!(
            proxy = %entry.cfg.name,
            %local_addr,
            "udp work conn; starting forwarder"
        );

        let work = maybe_limit(work, entry.limiter.clone());

        run_udp_session(
            work,
            local_addr,
            self.udp_packet_size,
            entry.proxy_protocol.map(|s| s.to_string()),
            cancel_rx,
        )
        .await
    }
}
