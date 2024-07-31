use actix_web::{test, web, App};
use contexter::config::Config;
use contexter::server::{AppState, ProjectContentResponse, ProjectListResponse, ProjectMetadata};
use contexter::server_handlers::{get_project_metadata, list_projects, run_contexter};

use sha2::{Digest, Sha256};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;

const TEST_API_KEY: &str = "test_api_key";

fn hash_api_key(key: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(key.as_bytes());
    hex::encode(hasher.finalize())
}

async fn setup_test_app() -> (Config, web::Data<AppState>) {
    let mut config = Config::default();
    config.add_project(
        "test_project".to_string(),
        PathBuf::from("/path/to/test_project"),
    );

    // Add a valid API key to the configuration
    config
        .api_keys
        .insert("test_key_name".to_string(), hash_api_key(TEST_API_KEY));

    let app_state = web::Data::new(AppState {
        config: Arc::new(RwLock::new(config.clone())),
    });

    (config, app_state)
}

#[actix_rt::test]
async fn test_list_projects() {
    let (_, app_state) = setup_test_app().await;

    let app = test::init_service(
        App::new()
            .app_data(app_state)
            .route("/api/v1/projects", web::get().to(list_projects)),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/api/v1/projects")
        .insert_header(("X-API-Key", TEST_API_KEY))
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 200);

    let body = test::read_body(resp).await;
    let resp: ProjectListResponse = serde_json::from_slice(&body).unwrap();

    assert_eq!(resp.projects.len(), 1);
    assert_eq!(resp.projects[0].name, "test_project");
    assert_eq!(resp.projects[0].path, "/path/to/test_project");
}

#[actix_rt::test]
async fn test_get_project_metadata() {
    let (_, app_state) = setup_test_app().await;

    let app = test::init_service(App::new().app_data(app_state).route(
        "/api/v1/projects/{name}",
        web::get().to(get_project_metadata),
    ))
    .await;

    let req = test::TestRequest::get()
        .uri("/api/v1/projects/test_project")
        .insert_header(("X-API-Key", TEST_API_KEY))
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 200);

    let body = test::read_body(resp).await;
    let resp: ProjectMetadata = serde_json::from_slice(&body).unwrap();

    assert_eq!(resp.name, "test_project");
    assert_eq!(resp.path, "/path/to/test_project");
    assert!(resp.files.is_empty());
}

#[actix_rt::test]
async fn test_run_contexter() {
    let (_, app_state) = setup_test_app().await;

    let app = test::init_service(
        App::new()
            .app_data(app_state)
            .route("/api/v1/projects/{name}", web::post().to(run_contexter)),
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

    assert_eq!(resp.status(), 200);

    let body = test::read_body(resp).await;
    let resp: ProjectContentResponse = serde_json::from_slice(&body).unwrap();

    assert!(resp.content.is_empty());
}

#[actix_rt::test]
async fn test_unauthorized_access() {
    let (_, app_state) = setup_test_app().await;

    let app = test::init_service(
        App::new()
            .app_data(app_state)
            .route("/api/v1/projects", web::get().to(list_projects)),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/api/v1/projects")
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 401);
}

#[actix_rt::test]
async fn test_project_not_found() {
    let (_, app_state) = setup_test_app().await;

    let app = test::init_service(App::new().app_data(app_state).route(
        "/api/v1/projects/{name}",
        web::get().to(get_project_metadata),
    ))
    .await;

    let req = test::TestRequest::get()
        .uri("/api/v1/projects/nonexistent_project")
        .insert_header(("X-API-Key", TEST_API_KEY))
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 404);
}
