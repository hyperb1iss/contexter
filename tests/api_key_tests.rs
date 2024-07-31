use actix_web::test;
use contexter::config::Config;
use contexter::utils::{generate_api_key, hash_api_key, validate_api_key};

#[tokio::test]
async fn test_api_key_generation() {
    let mut config = Config::default();
    let name = "test_key_name";
    let key = generate_api_key();
    let hashed_key = hash_api_key(&key);
    config.add_api_key(name.to_string(), hashed_key);

    assert_eq!(config.api_keys.len(), 1);
    assert!(config.api_keys.contains_key(name));
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
    let name = "test_key_name";
    let key = generate_api_key();
    let hashed_key = hash_api_key(&key);
    config.add_api_key(name.to_string(), hashed_key);

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
    let name1 = "test_key_name1";
    let name2 = "test_key_name2";
    let key1 = generate_api_key();
    let key2 = generate_api_key();
    let hashed_key1 = hash_api_key(&key1);
    let hashed_key2 = hash_api_key(&key2);

    config.add_api_key(name1.to_string(), hashed_key1);
    config.add_api_key(name2.to_string(), hashed_key2);

    assert_eq!(config.api_keys.len(), 2);
    assert!(config.api_keys.contains_key(name1));
    assert!(config.api_keys.contains_key(name2));
}

#[tokio::test]
async fn test_remove_api_key() {
    let mut config = Config::default();
    let name = "test_key_name";
    let key = generate_api_key();
    let hashed_key = hash_api_key(&key);

    config.add_api_key(name.to_string(), hashed_key);
    assert_eq!(config.api_keys.len(), 1);

    config.remove_api_key(name);
    assert_eq!(config.api_keys.len(), 0);
}
