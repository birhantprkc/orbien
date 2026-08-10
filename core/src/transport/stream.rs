use tokio::io::{AsyncRead, AsyncWrite};

pub trait AsyncStream: AsyncRead + AsyncWrite + Send + Unpin {}
impl<T> AsyncStream for T where T: AsyncRead + AsyncWrite + Send + Unpin {}
pub type DynStream = Box<dyn AsyncStream>;
pub fn boxed_stream<T>(stream: T) -> DynStream
where
    T: AsyncStream + 'static,
{
    Box::new(stream)
}
