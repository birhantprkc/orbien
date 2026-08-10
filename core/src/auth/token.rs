use md5::{Digest, Md5};

pub fn get_auth_key(token: &str, timestamp: i64) -> String {
    let mut hasher = Md5::new();
    hasher.update(token.as_bytes());
    hasher.update(timestamp.to_string().as_bytes());
    hex::encode(hasher.finalize())
}

pub fn verify_login(token: &str, privilege_key: &str, timestamp: i64) -> bool {
    if token.is_empty() {
        return true;
    }
    get_auth_key(token, timestamp) == privilege_key
}
