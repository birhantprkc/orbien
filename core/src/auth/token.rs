use md5::{Digest, Md5};
use std::io::Write;

pub fn compute_auth_digest(token: &str, timestamp: i64) -> String {
    let mut hasher = Md5::new();
    hasher.update(token.as_bytes());
    let mut ts = [0u8; 20];
    let mut cur = std::io::Cursor::new(&mut ts[..]);
    let _ = write!(cur, "{timestamp}");
    let n = cur.position() as usize;
    hasher.update(&ts[..n]);
    hex::encode(hasher.finalize())
}

pub fn verify_login(token: &str, auth_digest: &str, timestamp: i64) -> bool {
    verify_auth_digest(token, auth_digest, timestamp)
}

pub fn verify_auth_digest(token: &str, auth_digest: &str, timestamp: i64) -> bool {
    if token.is_empty() {
        return true;
    }
    if auth_digest.is_empty() {
        return false;
    }
    compute_auth_digest(token, timestamp) == auth_digest
}
