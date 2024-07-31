use actix_web::{test, web, App};
use contexter::config::Config;
use contexter::server::{
    get_project_content, list_projects, AppState, ProjectContentResponse, ProjectListResponse,
};
use std::sync::Arc;
use tokio::sync::RwLock;

async fn setup_test_app() -> (web::Data<AppState>, tempfile::TempDir) {
    let dir = tempfile::tempdir().unwrap();
    let project_path = dir.path().join("test_project");
    std::fs::create_dir(&project_path).unwrap();
    std::fs::write(project_path.join("test.txt"), "Test content").unwrap();

    let mut config = Config::default();
    config.add_project("test".to_string(), project_path);
    config.add_api_key("test_key".to_string());

    let app_state = web::Data::new(AppState {
        config: Arc::new(RwLock::new(config)),
    });

    (app_state, dir)
}

#[actix_rt::test]
async fn test_list_projects() {
    let (app_state, _dir) = setup_test_app().await;

    let app = test::init_service(
        App::new()
            .app_data(app_state.clone())
            .route("/projects", web::get().to(list_projects)),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/projects?api_key=test_key")
        .to_request();
    let resp: ProjectListResponse = test::call_and_read_body_json(&app, req).await;

    assert_eq!(resp.projects, vec!["test"]);
}

#[actix_rt::test]
async fn test_get_project_content() {
    let (app_state, _dir) = setup_test_app().await;

    let app = test::init_service(
        App::new()
            .app_data(app_state.clone())
            .route("/project/{name}", web::get().to(get_project_content)),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/project/test?api_key=test_key")
        .to_request();
    let resp: ProjectContentResponse = test::call_and_read_body_json(&app, req).await;

    assert!(resp.content.contains("Test content"));
}
