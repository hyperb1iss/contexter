use actix_web::test;
use base64::{engine::general_purpose, Engine as _};
use contexter::config::Config;
use contexter::utils::validate_api_key;
use rand::rngs::OsRng;
use rand::RngCore;
use sha2::{Digest, Sha256};

fn generate_api_key() -> String {
    let mut key = [0u8; 32];
    OsRng.fill_bytes(&mut key);
    general_purpose::URL_SAFE_NO_PAD.encode(key)
}

fn hash_api_key(key: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(key.as_bytes());
    hex::encode(hasher.finalize())
}

#[tokio::test]
async fn test_api_key_generation() {
    let key = generate_api_key();
    assert_eq!(key.len(), 43); // Base64 encoding of 32 bytes
    assert!(key
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_'));
}

#[tokio::test]
async fn test_api_key_hashing() {
    let key = "test_key";
    let hashed_key = hash_api_key(key);
    assert_eq!(hashed_key.len(), 64); // SHA-256 hash is 32 bytes, hex-encoded to 64 characters
    assert!(hashed_key.chars().all(|c| c.is_ascii_hexdigit()));
}

#[tokio::test]
async fn test_api_key_validation() {
    let mut config = Config::default();
    let key = generate_api_key();
    let hashed_key = hash_api_key(&key);
    config.add_api_key(hashed_key);

    let req = test::TestRequest::default()
        .insert_header(("X-API-Key", key.as_str()))
        .to_http_request();
    assert!(validate_api_key(&req, &config).await);

    let invalid_key = "invalid_key";
    let req = test::TestRequest::default()
        .insert_header(("X-API-Key", invalid_key))
        .to_http_request();
    assert!(!validate_api_key(&req, &config).await);
}

#[tokio::test]
async fn test_multiple_api_keys() {
    let mut config = Config::default();
    let key1 = generate_api_key();
    let key2 = generate_api_key();
    let hashed_key1 = hash_api_key(&key1);
    let hashed_key2 = hash_api_key(&key2);

    config.add_api_key(hashed_key1.clone());
    config.add_api_key(hashed_key2.clone());

    assert_eq!(config.api_keys.len(), 2);
    assert!(config.api_keys.contains(&hashed_key1));
    assert!(config.api_keys.contains(&hashed_key2));
}

#[tokio::test]
async fn test_remove_api_key() {
    let mut config = Config::default();
    let key = generate_api_key();
    let hashed_key = hash_api_key(&key);

    config.add_api_key(hashed_key.clone());
    assert_eq!(config.api_keys.len(), 1);

    config.remove_api_key(&hashed_key);
    assert_eq!(config.api_keys.len(), 0);
}
