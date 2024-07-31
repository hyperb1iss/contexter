use crate::config::Config;
use crate::contexter::{concatenate_files, gather_relevant_files};
use actix_web::{middleware::Logger, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use constant_time_eq::constant_time_eq;
use log::{debug, error, info, warn, LevelFilter};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct AppState {
    pub config: Arc<RwLock<Config>>,
}

#[derive(Serialize, Deserialize)]
pub struct ProjectListResponse {
    pub projects: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ProjectContentResponse {
    pub content: String,
}

#[derive(Serialize, Deserialize)]
pub struct ProjectFilesResponse {
    pub files: Vec<String>,
}

#[derive(Deserialize)]
pub struct ContexterRequest {
    pub files: Option<Vec<String>>,
    pub path: Option<String>,
}

fn hash_api_key(key: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(key.as_bytes());
    hex::encode(hasher.finalize())
}

pub async fn validate_api_key(config: &Config, api_key: &str) -> bool {
    let hashed_key = hash_api_key(api_key);
    config
        .api_keys
        .iter()
        .any(|stored_key| constant_time_eq(stored_key.as_bytes(), hashed_key.as_bytes()))
}

pub async fn list_projects(req: HttpRequest, data: web::Data<AppState>) -> impl Responder {
    let config = data.config.read().await;
    if let Some(api_key) = req.headers().get("X-API-Key") {
        if !validate_api_key(&config, api_key.to_str().unwrap_or("")).await {
            warn!("Unauthorized access attempt to list projects");
            return HttpResponse::Unauthorized().finish();
        }
    } else {
        warn!("Missing API key in request to list projects");
        return HttpResponse::Unauthorized().finish();
    }

    let projects: Vec<String> = config.projects.keys().cloned().collect();
    info!("Listed {} projects", projects.len());
    HttpResponse::Ok().json(ProjectListResponse { projects })
}

pub async fn get_project_content(
    req: HttpRequest,
    project_name: web::Path<String>,
    data: web::Data<AppState>,
) -> impl Responder {
    let config = data.config.read().await;
    if let Some(api_key) = req.headers().get("X-API-Key") {
        if !validate_api_key(&config, api_key.to_str().unwrap_or("")).await {
            warn!(
                "Unauthorized access attempt to get content for project: {}",
                project_name
            );
            return HttpResponse::Unauthorized().finish();
        }
    } else {
        warn!(
            "Missing API key in request to get content for project: {}",
            project_name
        );
        return HttpResponse::Unauthorized().finish();
    }

    let project_name = project_name.into_inner();

    if let Some(project_path) = config.projects.get(&project_name) {
        debug!("Gathering context for project: {}", project_name);
        match gather_relevant_files(project_path.to_str().unwrap(), vec![], vec![]) {
            Ok(files) => match concatenate_files(files) {
                Ok((content, _)) => {
                    info!(
                        "Successfully retrieved content for project: {}",
                        project_name
                    );
                    HttpResponse::Ok().json(ProjectContentResponse { content })
                }
                Err(e) => {
                    error!(
                        "Error concatenating files for project {}: {}",
                        project_name, e
                    );
                    HttpResponse::InternalServerError().finish()
                }
            },
            Err(e) => {
                error!("Error gathering files for project {}: {}", project_name, e);
                HttpResponse::InternalServerError().finish()
            }
        }
    } else {
        warn!("Project not found: {}", project_name);
        HttpResponse::NotFound().body(format!("Project '{}' not found", project_name))
    }
}

pub async fn list_project_files(
    req: HttpRequest,
    project_name: web::Path<String>,
    data: web::Data<AppState>,
) -> impl Responder {
    let config = data.config.read().await;
    if let Some(api_key) = req.headers().get("X-API-Key") {
        if !validate_api_key(&config, api_key.to_str().unwrap_or("")).await {
            warn!(
                "Unauthorized access attempt to list files for project: {}",
                project_name
            );
            return HttpResponse::Unauthorized().finish();
        }
    } else {
        warn!(
            "Missing API key in request to list files for project: {}",
            project_name
        );
        return HttpResponse::Unauthorized().finish();
    }

    let project_name = project_name.into_inner();

    if let Some(project_path) = config.projects.get(&project_name) {
        debug!("Listing files for project: {}", project_name);
        match gather_relevant_files(project_path.to_str().unwrap(), vec![], vec![]) {
            Ok(files) => {
                let file_paths: Vec<String> = files
                    .iter()
                    .map(|path| {
                        path.strip_prefix(project_path)
                            .unwrap()
                            .to_string_lossy()
                            .into_owned()
                    })
                    .collect();
                info!(
                    "Listed {} files for project: {}",
                    file_paths.len(),
                    project_name
                );
                HttpResponse::Ok().json(ProjectFilesResponse { files: file_paths })
            }
            Err(e) => {
                error!("Error gathering files for project {}: {}", project_name, e);
                HttpResponse::InternalServerError().finish()
            }
        }
    } else {
        warn!("Project not found: {}", project_name);
        HttpResponse::NotFound().body(format!("Project '{}' not found", project_name))
    }
}

pub async fn run_contexter(
    req: HttpRequest,
    project_name: web::Path<String>,
    contexter_req: web::Json<ContexterRequest>,
    data: web::Data<AppState>,
) -> impl Responder {
    let config = data.config.read().await;
    if let Some(api_key) = req.headers().get("X-API-Key") {
        if !validate_api_key(&config, api_key.to_str().unwrap_or("")).await {
            warn!(
                "Unauthorized access attempt to run contexter for project: {}",
                project_name
            );
            return HttpResponse::Unauthorized().finish();
        }
    } else {
        warn!(
            "Missing API key in request to run contexter for project: {}",
            project_name
        );
        return HttpResponse::Unauthorized().finish();
    }

    let project_name = project_name.into_inner();

    if let Some(project_path) = config.projects.get(&project_name) {
        let base_path = project_path.clone();
        let files_to_process = if let Some(files) = &contexter_req.files {
            debug!(
                "Running contexter on specific files for project: {}",
                project_name
            );
            files.iter().map(|f| base_path.join(f)).collect()
        } else if let Some(path) = &contexter_req.path {
            debug!(
                "Running contexter on path '{}' for project: {}",
                path, project_name
            );
            let full_path = base_path.join(path);
            match gather_relevant_files(full_path.to_str().unwrap(), vec![], vec![]) {
                Ok(files) => files,
                Err(e) => {
                    error!(
                        "Error gathering files for project {} at path {}: {}",
                        project_name, path, e
                    );
                    return HttpResponse::InternalServerError().finish();
                }
            }
        } else {
            warn!("Invalid contexter request for project: {}", project_name);
            return HttpResponse::BadRequest().body("Either 'files' or 'path' must be specified");
        };

        match concatenate_files(files_to_process) {
            Ok((content, processed_files)) => {
                info!(
                    "Successfully ran contexter on {} files for project: {}",
                    processed_files.len(),
                    project_name
                );
                HttpResponse::Ok().json(ProjectContentResponse { content })
            }
            Err(e) => {
                error!(
                    "Error concatenating files for project {}: {}",
                    project_name, e
                );
                HttpResponse::InternalServerError().finish()
            }
        }
    } else {
        warn!("Project not found: {}", project_name);
        HttpResponse::NotFound().body(format!("Project '{}' not found", project_name))
    }
}

pub async fn run_server(config: Config, log_level: LevelFilter) -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        config: Arc::new(RwLock::new(config)),
    });

    // Read configuration values before starting the server
    let listen_address = app_state.config.read().await.listen_address.clone();
    let port = app_state.config.read().await.port;

    info!("Starting server on {}:{}", listen_address, port);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(app_state.clone())
            .route("/projects", web::get().to(list_projects))
            .route("/project/{name}", web::get().to(get_project_content))
            .route("/project/{name}/files", web::get().to(list_project_files))
            .route("/project/{name}/contexter", web::post().to(run_contexter))
    })
    .bind((listen_address, port))?
    .run()
    .await
}
