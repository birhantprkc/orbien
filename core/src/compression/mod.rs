mod algo;
mod stream;

use crate::limit::{maybe_limit, BandwidthLimiter};
use crate::transport::DynStream;
use std::sync::Arc;

pub use algo::CompressionAlgo;
pub use stream::{maybe_compress, Lz4Stream};

pub fn wrap_data_conn(
    stream: DynStream,
    limiter: Option<Arc<BandwidthLimiter>>,
    compression: CompressionAlgo,
) -> DynStream {
    maybe_compress(maybe_limit(stream, limiter), compression)
}
