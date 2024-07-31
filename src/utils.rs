use actix_web::HttpRequest;
use constant_time_eq::constant_time_eq;
use rand::rngs::OsRng;
use rand::RngCore;
use sha2::{Digest, Sha256};
use base64::{Engine as _, engine::general_purpose};
use crate::config::Config;

pub fn generate_api_key() -> String {
    let mut key = [0u8; 32];
    OsRng.fill_bytes(&mut key);
    general_purpose::URL_SAFE_NO_PAD.encode(key)
}

pub fn hash_api_key(key: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(key.as_bytes());
    hex::encode(hasher.finalize())
}

pub async fn validate_api_key(req: &HttpRequest, config: &Config) -> bool {
    if let Some(api_key) = req.headers().get("X-API-Key") {
        let hashed_key = hash_api_key(api_key.to_str().unwrap_or(""));
        config
            .api_keys
            .iter()
            .any(|stored_key| constant_time_eq(stored_key.as_bytes(), hashed_key.as_bytes()))
    } else {
        false
    }
}