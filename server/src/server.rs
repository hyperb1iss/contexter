use crate::config::Config;
use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct AppState {
    pub config: Arc<RwLock<Config>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectListResponse {
    pub projects: Vec<ProjectSummary>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectSummary {
    pub name: String,
    pub path: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectMetadata {
    pub name: String,
    pub path: String,
    pub files: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ProjectContentResponse {
    pub content: String,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

pub fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .route(
                "/projects",
                web::get().to(crate::server_handlers::list_projects),
            )
            .route(
                "/projects/{name}",
                web::get().to(crate::server_handlers::get_project_metadata),
            )
            .route(
                "/projects/{name}",
                web::post().to(crate::server_handlers::run_contexter),
            ),
    );
}

pub async fn run_server(config: Config) -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        config: Arc::new(RwLock::new(config)),
    });

    let listen_address = app_state.config.read().await.listen_address.clone();
    let port = app_state.config.read().await.port;

    HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .app_data(app_state.clone())
            .configure(config_routes)
    })
    .bind((listen_address, port))?
    .run()
    .await
}
