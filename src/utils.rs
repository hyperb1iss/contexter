use crate::config::Config;
use actix_web::HttpRequest;
use base64::{engine::general_purpose, Engine as _};
use constant_time_eq::constant_time_eq;
use rand::rngs::OsRng;
use rand::RngCore;
use sha2::{Digest, Sha256};

/// Generates a new API key.
pub fn generate_api_key() -> String {
    let mut key = [0u8; 32];
    OsRng.fill_bytes(&mut key);
    general_purpose::URL_SAFE_NO_PAD.encode(key)
}

/// Hashes the given API key using SHA-256.
pub fn hash_api_key(key: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(key.as_bytes());
    hex::encode(hasher.finalize())
}

/// Validates the API key provided in the request against the stored API keys in the configuration.
pub async fn validate_api_key(req: &HttpRequest, config: &Config) -> bool {
    if config.api_keys.is_empty() {
        return false;
    }

    if let Some(api_key) = req.headers().get("X-API-Key") {
        let api_key = api_key.to_str().unwrap_or("");
        let hashed_key = hash_api_key(api_key);
        config
            .api_keys
            .values()
            .any(|stored_key| constant_time_eq(stored_key.as_bytes(), hashed_key.as_bytes()))
    } else {
        false
    }
}
