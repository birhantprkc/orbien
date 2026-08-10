use anyhow::{anyhow, Result};
use orbien_core::msg::{self, Message};
use orbien_core::transport::DynStream;
use orbien_core::udp::{forwarder, spawn_work_ping, CHANNEL_CAP};
use std::net::SocketAddr;
use tokio::io::AsyncRead;
use tokio::sync::{mpsc, oneshot};

pub async fn run_udp_session(
    work: DynStream,
    local_addr: SocketAddr,
    packet_size: usize,
    proxy_protocol_version: Option<String>,
    mut cancel_rx: oneshot::Receiver<()>,
) -> Result<()> {
    let (reader, mut writer) = tokio::io::split(work);

    let (to_work_tx, mut to_work_rx) = mpsc::channel::<Message>(CHANNEL_CAP);
    let (from_work_tx, from_work_rx) = mpsc::channel(CHANNEL_CAP);

    let ping = spawn_work_ping(to_work_tx.clone());
    let forward = {
        let tx = to_work_tx;
        tokio::spawn(async move {
            forwarder(
                local_addr,
                from_work_rx,
                tx,
                packet_size,
                proxy_protocol_version,
            )
            .await;
        })
    };

    let (fail_tx, mut fail_rx) = mpsc::channel::<anyhow::Error>(1);
    let from_work_tx_r = from_work_tx;
    let fail_r = fail_tx;
    let reader_task = tokio::spawn(async move {
        work_reader(reader, from_work_tx_r, fail_r).await;
    });

    let result = loop {
        tokio::select! {
            _ = &mut cancel_rx => {
                tracing::info!(%local_addr, "udp session replaced");
                break Ok(());
            }
            err = fail_rx.recv() => {
                break match err {
                    Some(e) => Err(e),
                    None => Ok(()),
                };
            }
            out = to_work_rx.recv() => {
                match out {
                    Some(m) => {
                        if let Err(e) = msg::write_msg(&mut writer, &m).await {
                            break Err(anyhow!("udp work write: {e}"));
                        }
                    }
                    None => break Ok(()),
                }
            }
        }
    };

    reader_task.abort();
    ping.abort();
    forward.abort();
    let _ = reader_task.await;
    let _ = ping.await;
    let _ = forward.await;
    result
}

async fn work_reader<R: AsyncRead + Unpin + Send + 'static>(
    mut reader: R,
    from_work_tx: mpsc::Sender<orbien_core::msg::UdpPacket>,
    fail_tx: mpsc::Sender<anyhow::Error>,
) {
    loop {
        match msg::read_msg(&mut reader).await {
            Ok(Message::UdpPacket(pkt)) => {
                tracing::trace!(len = pkt.content.len(), "udp packet from work");
                let _ = from_work_tx.try_send(pkt);
            }
            Ok(Message::Ping(_)) => {}
            Ok(other) => {
                tracing::debug!(ty = other.type_byte(), "udp work unexpected message");
            }
            Err(e) => {
                let _ = fail_tx.send(anyhow!("udp work read: {e}")).await;
                return;
            }
        }
    }
}
