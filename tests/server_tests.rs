use actix_web::{test, web, App};
use base64::{engine::general_purpose, Engine as _};
use contexter::config::Config;
use contexter::server::{
    get_project_content, list_project_files, list_projects, run_contexter, AppState,
    ProjectContentResponse, ProjectFilesResponse, ProjectListResponse,
};
use rand::rngs::OsRng;
use rand::RngCore;
use sha2::{Digest, Sha256};
use std::sync::Arc;
use tokio::sync::RwLock;

async fn setup_test_app() -> (web::Data<AppState>, tempfile::TempDir, String) {
    let dir = tempfile::tempdir().unwrap();
    let project_path = dir.path().join("test_project");
    std::fs::create_dir(&project_path).unwrap();
    std::fs::write(project_path.join("test.txt"), "Test content").unwrap();

    let mut config = Config::default();
    config.add_project("test".to_string(), project_path);
    let api_key = generate_api_key();
    let hashed_key = hash_api_key(&api_key);
    config.add_api_key(hashed_key);

    let app_state = web::Data::new(AppState {
        config: Arc::new(RwLock::new(config)),
    });

    (app_state, dir, api_key)
}

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

#[actix_rt::test]
async fn test_list_projects() {
    let (app_state, _dir, api_key) = setup_test_app().await;

    let app = test::init_service(
        App::new()
            .app_data(app_state.clone())
            .route("/projects", web::get().to(list_projects)),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/projects")
        .insert_header(("X-API-Key", api_key))
        .to_request();
    let resp: ProjectListResponse = test::call_and_read_body_json(&app, req).await;

    assert_eq!(resp.projects, vec!["test"]);
}

#[actix_rt::test]
async fn test_list_projects_unauthorized() {
    let (app_state, _dir, _) = setup_test_app().await;

    let app = test::init_service(
        App::new()
            .app_data(app_state.clone())
            .route("/projects", web::get().to(list_projects)),
    )
    .await;

    let req = test::TestRequest::get().uri("/projects").to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 401);
}

#[actix_rt::test]
async fn test_get_project_content() {
    let (app_state, _dir, api_key) = setup_test_app().await;

    let app = test::init_service(
        App::new()
            .app_data(app_state.clone())
            .route("/project/{name}", web::get().to(get_project_content)),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/project/test")
        .insert_header(("X-API-Key", api_key))
        .to_request();
    let resp: ProjectContentResponse = test::call_and_read_body_json(&app, req).await;

    assert!(resp.content.contains("Test content"));
}

#[actix_rt::test]
async fn test_get_project_content_unauthorized() {
    let (app_state, _dir, _) = setup_test_app().await;

    let app = test::init_service(
        App::new()
            .app_data(app_state.clone())
            .route("/project/{name}", web::get().to(get_project_content)),
    )
    .await;

    let req = test::TestRequest::get().uri("/project/test").to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 401);
}

#[actix_rt::test]
async fn test_get_project_content_not_found() {
    let (app_state, _dir, api_key) = setup_test_app().await;

    let app = test::init_service(
        App::new()
            .app_data(app_state.clone())
            .route("/project/{name}", web::get().to(get_project_content)),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/project/nonexistent")
        .insert_header(("X-API-Key", api_key))
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 404);
}

#[actix_rt::test]
async fn test_list_project_files() {
    let (app_state, dir, api_key) = setup_test_app().await;

    // Create some test files
    std::fs::write(dir.path().join("test_project/file1.txt"), "Test content 1").unwrap();
    std::fs::write(dir.path().join("test_project/file2.txt"), "Test content 2").unwrap();
    std::fs::create_dir(dir.path().join("test_project/subfolder")).unwrap();
    std::fs::write(
        dir.path().join("test_project/subfolder/file3.txt"),
        "Test content 3",
    )
    .unwrap();

    let app = test::init_service(
        App::new()
            .app_data(app_state.clone())
            .route("/project/{name}/files", web::get().to(list_project_files)),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/project/test/files")
        .insert_header(("X-API-Key", api_key))
        .to_request();
    let resp: ProjectFilesResponse = test::call_and_read_body_json(&app, req).await;

    assert_eq!(resp.files.len(), 4);
    assert!(resp.files.contains(&"test.txt".to_string()));
    assert!(resp.files.contains(&"file1.txt".to_string()));
    assert!(resp.files.contains(&"file2.txt".to_string()));
    assert!(resp.files.contains(&"subfolder/file3.txt".to_string()));
}

#[actix_rt::test]
async fn test_run_contexter_with_files() {
    let (app_state, dir, api_key) = setup_test_app().await;

    // Create some test files
    std::fs::write(dir.path().join("test_project/file1.txt"), "Test content 1").unwrap();
    std::fs::write(dir.path().join("test_project/file2.txt"), "Test content 2").unwrap();

    let app = test::init_service(
        App::new()
            .app_data(app_state.clone())
            .route("/project/{name}/contexter", web::post().to(run_contexter)),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/project/test/contexter")
        .insert_header(("X-API-Key", api_key))
        .set_json(serde_json::json!({
            "files": ["file1.txt", "file2.txt"]
        }))
        .to_request();
    let resp: ProjectContentResponse = test::call_and_read_body_json(&app, req).await;

    assert!(resp.content.contains("Test content 1"));
    assert!(resp.content.contains("Test content 2"));
}

#[actix_rt::test]
async fn test_run_contexter_with_path() {
    let (app_state, dir, api_key) = setup_test_app().await;

    // Create some test files
    std::fs::create_dir(dir.path().join("test_project/subfolder")).unwrap();
    std::fs::write(
        dir.path().join("test_project/subfolder/file1.txt"),
        "Test content 1",
    )
    .unwrap();
    std::fs::write(
        dir.path().join("test_project/subfolder/file2.txt"),
        "Test content 2",
    )
    .unwrap();

    let app = test::init_service(
        App::new()
            .app_data(app_state.clone())
            .route("/project/{name}/contexter", web::post().to(run_contexter)),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/project/test/contexter")
        .insert_header(("X-API-Key", api_key))
        .set_json(serde_json::json!({
            "path": "subfolder"
        }))
        .to_request();
    let resp: ProjectContentResponse = test::call_and_read_body_json(&app, req).await;

    assert!(resp.content.contains("Test content 1"));
    assert!(resp.content.contains("Test content 2"));
}

#[actix_rt::test]
async fn test_run_contexter_invalid_request() {
    let (app_state, _dir, api_key) = setup_test_app().await;

    let app = test::init_service(
        App::new()
            .app_data(app_state.clone())
            .route("/project/{name}/contexter", web::post().to(run_contexter)),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/project/test/contexter")
        .insert_header(("X-API-Key", api_key))
        .set_json(serde_json::json!({}))
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 400);
}
