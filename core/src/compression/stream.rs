use super::CompressionAlgo;
use crate::transport::{boxed_stream, DynStream};
use pin_project_lite::pin_project;
use std::io;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};

const MAX_PLAIN_CHUNK: usize = 64 * 1024;
const MAX_FRAME_PAYLOAD: usize = 1 + MAX_PLAIN_CHUNK + 64;
const MAX_PLAIN_DECODE: usize = MAX_PLAIN_CHUNK;

const KIND_RAW: u8 = 0;
const KIND_LZ4: u8 = 1;

pin_project! {
    pub struct Lz4Stream<S> {
        #[pin]
        inner: S,
        write: WriteState,
        read: ReadState,
    }
}

struct WriteState {
    pending: Vec<u8>,
    pending_off: usize,
    acked: Option<usize>,
    finished: bool,
    compress_buf: Vec<u8>,
}

impl Default for WriteState {
    fn default() -> Self {
        Self {
            pending: Vec::new(),
            pending_off: 0,
            acked: None,
            finished: false,
            compress_buf: Vec::new(),
        }
    }
}

enum ReadPhase {
    Len { got: usize, buf: [u8; 4] },
    Body { need: usize, buf: Vec<u8> },
    Deliver { data: Vec<u8>, off: usize },
}

struct ReadState {
    phase: ReadPhase,
}

impl Default for ReadState {
    fn default() -> Self {
        Self {
            phase: ReadPhase::Len {
                got: 0,
                buf: [0; 4],
            },
        }
    }
}

impl<S> Lz4Stream<S> {
    pub fn new(inner: S) -> Self {
        Self {
            inner,
            write: WriteState::default(),
            read: ReadState::default(),
        }
    }

    fn decode_frame(body: &[u8]) -> io::Result<Vec<u8>> {
        if body.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "empty compression frame body",
            ));
        }
        let kind = body[0];
        let payload = &body[1..];
        match kind {
            KIND_RAW => {
                if payload.len() > MAX_PLAIN_DECODE {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "raw frame plaintext too large",
                    ));
                }
                Ok(payload.to_vec())
            }
            KIND_LZ4 => {
                if payload.len() < 4 {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "truncated lz4 payload",
                    ));
                }
                let declared = u32::from_le_bytes(payload[..4].try_into().unwrap()) as usize;
                if declared == 0 || declared > MAX_PLAIN_DECODE {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("lz4 declared size {declared} out of range"),
                    ));
                }
                lz4_flex::block::decompress_size_prepended(payload).map_err(|e| {
                    io::Error::new(io::ErrorKind::InvalidData, format!("lz4 decompress: {e}"))
                })
            }
            other => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("unknown compression frame kind {other}"),
            )),
        }
    }
}

impl WriteState {
    fn pending_remaining(&self) -> bool {
        self.pending_off < self.pending.len()
    }

    fn clear_pending(&mut self) {
        self.pending.clear();
        self.pending_off = 0;
    }

    fn encode_into_pending(&mut self, plain: &[u8]) -> io::Result<()> {
        debug_assert!(!plain.is_empty());
        debug_assert!(plain.len() <= MAX_PLAIN_CHUNK);

        self.compress_buf.clear();
        let max_comp = lz4_flex::block::get_maximum_output_size(plain.len());
        self.compress_buf.resize(4 + max_comp, 0);
        let written =
            lz4_flex::block::compress_into(plain, &mut self.compress_buf[4..]).map_err(|e| {
                io::Error::new(io::ErrorKind::InvalidData, format!("lz4 compress: {e}"))
            })?;
        self.compress_buf[0..4].copy_from_slice(&(plain.len() as u32).to_le_bytes());
        self.compress_buf.truncate(4 + written);

        let use_lz4 = self.compress_buf.len() < plain.len();
        let (kind, payload_len) = if use_lz4 {
            (KIND_LZ4, self.compress_buf.len())
        } else {
            (KIND_RAW, plain.len())
        };

        let body_len = 1 + payload_len;
        if body_len > MAX_FRAME_PAYLOAD {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "compression frame payload too large",
            ));
        }

        self.pending.clear();
        self.pending.reserve(4 + body_len);
        self.pending
            .extend_from_slice(&(body_len as u32).to_le_bytes());
        self.pending.push(kind);
        if use_lz4 {
            self.pending.extend_from_slice(&self.compress_buf);
        } else {
            self.pending.extend_from_slice(plain);
        }
        self.pending_off = 0;
        Ok(())
    }
}

fn poll_drain_pending<W: AsyncWrite + Unpin>(
    mut inner: Pin<&mut W>,
    write: &mut WriteState,
    cx: &mut Context<'_>,
) -> Poll<io::Result<()>> {
    while write.pending_remaining() {
        match inner
            .as_mut()
            .poll_write(cx, &write.pending[write.pending_off..])
        {
            Poll::Ready(Ok(0)) => {
                return Poll::Ready(Err(io::Error::new(
                    io::ErrorKind::WriteZero,
                    "write zero while sending compression frame",
                )));
            }
            Poll::Ready(Ok(n)) => write.pending_off += n,
            Poll::Ready(Err(e)) => return Poll::Ready(Err(e)),
            Poll::Pending => return Poll::Pending,
        }
    }
    Poll::Ready(Ok(()))
}

impl<S: AsyncRead + Unpin> AsyncRead for Lz4Stream<S> {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        if buf.remaining() == 0 {
            return Poll::Ready(Ok(()));
        }

        let mut this = self.project();
        loop {
            match &mut this.read.phase {
                ReadPhase::Deliver { data, off } => {
                    let n = buf.remaining().min(data.len() - *off);
                    if n == 0 {
                        this.read.phase = ReadPhase::Len {
                            got: 0,
                            buf: [0; 4],
                        };
                        continue;
                    }
                    buf.put_slice(&data[*off..*off + n]);
                    *off += n;
                    if *off >= data.len() {
                        this.read.phase = ReadPhase::Len {
                            got: 0,
                            buf: [0; 4],
                        };
                    }
                    return Poll::Ready(Ok(()));
                }
                ReadPhase::Len { got, buf: len_buf } => {
                    while *got < 4 {
                        let mut tmp = [0u8; 4];
                        let mut rb = ReadBuf::new(&mut tmp[..4 - *got]);
                        match this.inner.as_mut().poll_read(cx, &mut rb) {
                            Poll::Ready(Ok(())) => {
                                let filled = rb.filled();
                                if filled.is_empty() {
                                    if *got == 0 {
                                        return Poll::Ready(Ok(()));
                                    }
                                    return Poll::Ready(Err(io::Error::new(
                                        io::ErrorKind::UnexpectedEof,
                                        "eof in compression frame length",
                                    )));
                                }
                                len_buf[*got..*got + filled.len()].copy_from_slice(filled);
                                *got += filled.len();
                            }
                            Poll::Ready(Err(e)) => return Poll::Ready(Err(e)),
                            Poll::Pending => return Poll::Pending,
                        }
                    }
                    let need = u32::from_le_bytes(*len_buf) as usize;
                    if need == 0 || need > MAX_FRAME_PAYLOAD {
                        return Poll::Ready(Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            format!("invalid compression frame length {need}"),
                        )));
                    }
                    this.read.phase = ReadPhase::Body {
                        need,
                        buf: Vec::with_capacity(need),
                    };
                }
                ReadPhase::Body { need, buf: body } => {
                    while body.len() < *need {
                        let mut tmp = [0u8; 8 * 1024];
                        let want = (*need - body.len()).min(tmp.len());
                        let mut rb = ReadBuf::new(&mut tmp[..want]);
                        match this.inner.as_mut().poll_read(cx, &mut rb) {
                            Poll::Ready(Ok(())) => {
                                let filled = rb.filled();
                                if filled.is_empty() {
                                    return Poll::Ready(Err(io::Error::new(
                                        io::ErrorKind::UnexpectedEof,
                                        "eof in compression frame body",
                                    )));
                                }
                                body.extend_from_slice(filled);
                            }
                            Poll::Ready(Err(e)) => return Poll::Ready(Err(e)),
                            Poll::Pending => return Poll::Pending,
                        }
                    }
                    let plain = Lz4Stream::<S>::decode_frame(body)?;
                    this.read.phase = ReadPhase::Deliver {
                        data: plain,
                        off: 0,
                    };
                }
            }
        }
    }
}

impl<S: AsyncWrite + Unpin> AsyncWrite for Lz4Stream<S> {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        let mut this = self.project();
        if this.write.finished {
            return Poll::Ready(Err(io::Error::new(
                io::ErrorKind::WriteZero,
                "compression stream already shut down",
            )));
        }

        loop {
            match poll_drain_pending(this.inner.as_mut(), this.write, cx) {
                Poll::Ready(Ok(())) => {}
                Poll::Ready(Err(e)) => return Poll::Ready(Err(e)),
                Poll::Pending => return Poll::Pending,
            }

            if let Some(acked) = this.write.acked.take() {
                this.write.clear_pending();
                return Poll::Ready(Ok(acked));
            }

            this.write.clear_pending();

            if buf.is_empty() {
                return Poll::Ready(Ok(0));
            }

            let take = buf.len().min(MAX_PLAIN_CHUNK);
            this.write.encode_into_pending(&buf[..take])?;
            this.write.acked = Some(take);
        }
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        let mut this = self.project();
        match poll_drain_pending(this.inner.as_mut(), this.write, cx) {
            Poll::Ready(Ok(())) => {}
            Poll::Ready(Err(e)) => return Poll::Ready(Err(e)),
            Poll::Pending => return Poll::Pending,
        }
        if this.write.acked.is_none() {
            this.write.clear_pending();
        }
        this.inner.as_mut().poll_flush(cx)
    }

    fn poll_shutdown(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        match self.as_mut().poll_flush(cx) {
            Poll::Ready(Ok(())) => {}
            Poll::Ready(Err(e)) => return Poll::Ready(Err(e)),
            Poll::Pending => return Poll::Pending,
        }
        let mut this = self.project();
        this.write.finished = true;
        this.inner.as_mut().poll_shutdown(cx)
    }
}

pub fn maybe_compress(stream: DynStream, algo: CompressionAlgo) -> DynStream {
    match algo {
        CompressionAlgo::None => stream,
        CompressionAlgo::Lz4 => boxed_stream(Lz4Stream::new(stream)),
    }
}
