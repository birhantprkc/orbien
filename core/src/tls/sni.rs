use anyhow::{anyhow, bail, Result};
use std::io;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, ReadBuf};

const MAX_RECORD: usize = 5 + 16 * 1024;

pub async fn peek_client_hello_sni<R: AsyncRead + Unpin>(
    reader: &mut R,
) -> Result<(String, Vec<u8>)> {
    let mut buf = Vec::with_capacity(1024);
    let mut tmp = [0u8; 2048];

    while buf.len() < 5 {
        let n = reader.read(&mut tmp).await?;
        if n == 0 {
            bail!("connection closed before TLS ClientHello");
        }
        buf.extend_from_slice(&tmp[..n]);
        if buf.len() > MAX_RECORD {
            bail!("TLS preamble too large");
        }
    }

    if buf[0] != 0x16 {
        bail!("not a TLS handshake record (type={:#x})", buf[0]);
    }

    let record_len = u16::from_be_bytes([buf[3], buf[4]]) as usize;
    let need = 5 + record_len;
    if need > MAX_RECORD {
        bail!("TLS record too large: {record_len}");
    }

    while buf.len() < need {
        let n = reader.read(&mut tmp).await?;
        if n == 0 {
            bail!("connection closed mid ClientHello");
        }
        buf.extend_from_slice(&tmp[..n]);
        if buf.len() > MAX_RECORD {
            bail!("TLS preamble too large");
        }
    }

    let (record, rest) = buf.split_at(need);
    let sni = extract_sni_from_handshake_record(record)
        .ok_or_else(|| anyhow!("TLS ClientHello missing SNI"))?;
    let mut prefix = record.to_vec();
    prefix.extend_from_slice(rest);
    Ok((sni, prefix))
}

pub(crate) fn extract_sni_from_handshake_record(record: &[u8]) -> Option<String> {
    if record.len() < 5 + 4 {
        return None;
    }
    let hs = &record[5..];
    if hs.is_empty() || hs[0] != 0x01 {
        return None;
    }
    if hs.len() < 4 {
        return None;
    }
    let hs_len = ((hs[1] as usize) << 16) | ((hs[2] as usize) << 8) | (hs[3] as usize);
    if hs.len() < 4 + hs_len {
        return None;
    }
    let body = &hs[4..4 + hs_len];

    if body.len() < 2 + 32 + 1 {
        return None;
    }
    let mut i = 2 + 32;
    let sid_len = body[i] as usize;
    i += 1 + sid_len;
    if body.len() < i + 2 {
        return None;
    }
    let cs_len = u16::from_be_bytes([body[i], body[i + 1]]) as usize;
    i += 2 + cs_len;
    if body.len() < i + 1 {
        return None;
    }
    let comp_len = body[i] as usize;
    i += 1 + comp_len;
    if body.len() < i + 2 {
        return None;
    }
    let ext_len = u16::from_be_bytes([body[i], body[i + 1]]) as usize;
    i += 2;
    if body.len() < i + ext_len {
        return None;
    }
    let exts = &body[i..i + ext_len];
    parse_sni_extension(exts)
}

fn parse_sni_extension(mut exts: &[u8]) -> Option<String> {
    while exts.len() >= 4 {
        let typ = u16::from_be_bytes([exts[0], exts[1]]);
        let len = u16::from_be_bytes([exts[2], exts[3]]) as usize;
        exts = &exts[4..];
        if exts.len() < len {
            return None;
        }
        let data = &exts[..len];
        exts = &exts[len..];
        if typ == 0x0000 {
            if data.len() < 2 {
                return None;
            }
            let list_len = u16::from_be_bytes([data[0], data[1]]) as usize;
            if data.len() < 2 + list_len {
                return None;
            }
            let mut list = &data[2..2 + list_len];
            while list.len() >= 3 {
                let name_type = list[0];
                let name_len = u16::from_be_bytes([list[1], list[2]]) as usize;
                list = &list[3..];
                if list.len() < name_len {
                    return None;
                }
                if name_type == 0 {
                    let host = std::str::from_utf8(&list[..name_len]).ok()?;
                    return Some(host.to_ascii_lowercase());
                }
                list = &list[name_len..];
            }
            return None;
        }
    }
    None
}

pub struct PrefixedStream<S> {
    prefix: Vec<u8>,
    pos: usize,
    inner: S,
}

impl<S> PrefixedStream<S> {
    pub fn new(prefix: Vec<u8>, inner: S) -> Self {
        Self {
            prefix,
            pos: 0,
            inner,
        }
    }
}

impl<S: AsyncRead + Unpin> AsyncRead for PrefixedStream<S> {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        let this = self.get_mut();
        if this.pos < this.prefix.len() {
            let remain = &this.prefix[this.pos..];
            let n = remain.len().min(buf.remaining());
            buf.put_slice(&remain[..n]);
            this.pos += n;
            return Poll::Ready(Ok(()));
        }
        Pin::new(&mut this.inner).poll_read(cx, buf)
    }
}

impl<S: AsyncWrite + Unpin> AsyncWrite for PrefixedStream<S> {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        Pin::new(&mut self.get_mut().inner).poll_write(cx, buf)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Pin::new(&mut self.get_mut().inner).poll_flush(cx)
    }

    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Pin::new(&mut self.get_mut().inner).poll_shutdown(cx)
    }
}
