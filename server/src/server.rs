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

#[derive(Serialize, Deserialize)]
pub struct RepositoryAnalysisResponse {
    pub project_name: String,
    pub total_components: usize,
    pub entry_points: Vec<String>,
    pub dependency_cycles: usize,
    pub most_connected_components: Vec<String>,
    pub topological_order: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct RepositoryMapResponse {
    pub project_name: String,
    pub map: String,
    pub insights: crate::repo_mapper::RepositoryInsights,
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
            )
            .route(
                "/projects/{name}/analyze",
                web::post().to(crate::server_handlers::analyze_repository),
            )
            .route(
                "/projects/{name}/map",
                web::get().to(crate::server_handlers::get_repository_map),
            ),
    );
}

#[allow(clippy::future_not_send)]
pub async fn run_server(config: Config) -> std::io::Result<()> {
    // Extract values before creating the app state to avoid Send issues
    let listen_address = config.listen_address.clone();
    let port = config.port;

    let app_state = web::Data::new(AppState {
        config: Arc::new(RwLock::new(config)),
    });

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
