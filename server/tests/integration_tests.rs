use actix_cors::Cors;
use actix_web::{test, web, App};
use contexter::config::Config;
use contexter::server::{AppState, ProjectContentResponse, ProjectListResponse, ProjectMetadata};
use contexter::server_handlers::{get_project_metadata, list_projects, run_contexter};

use env_logger::Env;
use log::{debug, info};
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Once;
use tempfile::TempDir;
use tokio::sync::RwLock;

const TEST_API_KEY: &str = "test_api_key";

// Ensure logger is initialized only once
static INIT: Once = Once::new();

fn initialize_logger() {
    INIT.call_once(|| {
        env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();
    });
}

fn hash_api_key(key: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(key.as_bytes());
    hex::encode(hasher.finalize())
}

async fn setup_test_app() -> (Config, web::Data<AppState>, TempDir) {
    let temp_dir = TempDir::new().unwrap();
    let test_project_path = temp_dir.path().join("test_project");
    std::fs::create_dir_all(&test_project_path).unwrap();

    // Create dummy files for the test project
    File::create(test_project_path.join("file1.rs"))
        .unwrap()
        .write_all(b"// test file1")
        .unwrap();
    std::fs::create_dir_all(test_project_path.join("subfolder")).unwrap();
    File::create(test_project_path.join("subfolder/file2.rs"))
        .unwrap()
        .write_all(b"// test file2")
        .unwrap();

    let mut config = Config::default();
    config.add_project("test_project".to_string(), test_project_path.clone());

    // Add a valid API key to the configuration
    config
        .api_keys
        .insert("test_key_name".to_string(), hash_api_key(TEST_API_KEY));

    let app_state = web::Data::new(AppState {
        config: Arc::new(RwLock::new(config.clone())),
    });

    (config, app_state, temp_dir)
}

fn assert_cors_headers(headers: &actix_web::http::header::HeaderMap) {
    assert!(
        headers.contains_key("access-control-allow-credentials")
            || headers.contains_key("access-control-expose-headers"),
        "CORS headers not found in response: {:?}",
        headers
    );
}

#[actix_rt::test]
async fn test_list_projects() {
    initialize_logger();
    info!("Running test_list_projects");

    let (_, app_state, _temp_dir) = setup_test_app().await;

    let app = test::init_service(
        App::new()
            .wrap(Cors::permissive())
            .app_data(app_state)
            .configure(contexter::server::config_routes),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/api/v1/projects")
        .insert_header(("X-API-Key", TEST_API_KEY))
        .to_request();
    let resp = test::call_service(&app, req).await;

    info!("Response status: {:?}", resp.status());
    debug!("Response headers: {:?}", resp.headers());

    assert_eq!(resp.status(), 200);
    assert_cors_headers(resp.headers());

    let body = test::read_body(resp).await;
    let resp: ProjectListResponse = serde_json::from_slice(&body).unwrap();

    assert_eq!(resp.projects.len(), 1);
    assert_eq!(resp.projects[0].name, "test_project");
}

#[actix_rt::test]
async fn test_get_project_metadata() {
    initialize_logger();
    info!("Running test_get_project_metadata");

    let (_, app_state, _temp_dir) = setup_test_app().await;

    let app = test::init_service(
        App::new()
            .wrap(Cors::permissive())
            .app_data(app_state)
            .configure(contexter::server::config_routes),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/api/v1/projects/test_project")
        .insert_header(("X-API-Key", TEST_API_KEY))
        .to_request();
    let resp = test::call_service(&app, req).await;

    info!("Response status: {:?}", resp.status());
    debug!("Response headers: {:?}", resp.headers());

    assert_eq!(resp.status(), 200);
    assert_cors_headers(resp.headers());

    let body = test::read_body(resp).await;
    let resp: ProjectMetadata = serde_json::from_slice(&body).unwrap();

    assert_eq!(resp.name, "test_project");
    assert!(!resp.files.is_empty());
    assert!(resp.files.contains(&"file1.rs".to_string()));
    assert!(resp.files.contains(&"subfolder/file2.rs".to_string()));
}

#[actix_rt::test]
async fn test_run_contexter() {
    initialize_logger();
    info!("Running test_run_contexter");

    let (_, app_state, _temp_dir) = setup_test_app().await;

    let app = test::init_service(
        App::new()
            .wrap(Cors::permissive())
            .app_data(app_state)
            .configure(contexter::server::config_routes),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/api/v1/projects/test_project")
        .insert_header(("X-API-Key", TEST_API_KEY))
        .set_json(&serde_json::json!({
            "paths": ["file1.rs", "subfolder"]
        }))
        .to_request();
    let resp = test::call_service(&app, req).await;

    info!("Response status: {:?}", resp.status());
    debug!("Response headers: {:?}", resp.headers());

    assert_eq!(resp.status(), 200);
    assert_cors_headers(resp.headers());

    let body = test::read_body(resp).await;
    let resp: ProjectContentResponse = serde_json::from_slice(&body).unwrap();

    assert!(!resp.content.is_empty());
    assert!(resp.content.contains("// test file1"));
    assert!(resp.content.contains("// test file2"));
}

#[actix_rt::test]
async fn test_unauthorized_access() {
    initialize_logger();
    info!("Running test_unauthorized_access");

    let (_, app_state, _temp_dir) = setup_test_app().await;

    let app = test::init_service(
        App::new()
            .wrap(Cors::permissive())
            .app_data(app_state)
            .configure(contexter::server::config_routes),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/api/v1/projects")
        .to_request();
    let resp = test::call_service(&app, req).await;

    info!("Response status: {:?}", resp.status());
    debug!("Response headers: {:?}", resp.headers());

    assert_eq!(resp.status(), 401);
    assert_cors_headers(resp.headers());
}

#[actix_rt::test]
async fn test_project_not_found() {
    initialize_logger();
    info!("Running test_project_not_found");

    let (_, app_state, _temp_dir) = setup_test_app().await;

    let app = test::init_service(
        App::new()
            .wrap(Cors::permissive())
            .app_data(app_state)
            .configure(contexter::server::config_routes),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/api/v1/projects/nonexistent_project")
        .insert_header(("X-API-Key", TEST_API_KEY))
        .to_request();
    let resp = test::call_service(&app, req).await;

    info!("Response status: {:?}", resp.status());
    debug!("Response headers: {:?}", resp.headers());

    assert_eq!(resp.status(), 404);
    assert_cors_headers(resp.headers());
}
