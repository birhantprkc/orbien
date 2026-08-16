use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

pub fn compute_auth_digest(token: &str, timestamp: i64) -> String {
    let mut mac =
        HmacSha256::new_from_slice(token.as_bytes()).expect("HMAC-SHA256 accepts any key length");
    mac.update(timestamp.to_string().as_bytes());
    hex::encode(mac.finalize().into_bytes())
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
    let Ok(expected) = hex::decode(auth_digest) else {
        return false;
    };
    let Ok(mut mac) = HmacSha256::new_from_slice(token.as_bytes()) else {
        return false;
    };
    mac.update(timestamp.to_string().as_bytes());
    mac.verify_slice(&expected).is_ok()
}
