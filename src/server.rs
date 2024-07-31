use crate::config::Config;
use crate::contexter::{concatenate_files, gather_relevant_files};
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use constant_time_eq::constant_time_eq;
use log::{debug, error, info, warn};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::sync::Arc;
use tokio::sync::RwLock;
use serde_json;

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

#[derive(Deserialize)]
pub struct ContexterRequest {
    pub paths: Option<Vec<String>>,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
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

pub async fn list_projects(
    req: HttpRequest,
    data: web::Data<AppState>,
) -> impl Responder {
    let config = data.config.read().await;
    if let Some(api_key) = req.headers().get("X-API-Key") {
        if !validate_api_key(&config, api_key.to_str().unwrap_or("")).await {
            return HttpResponse::Unauthorized().json(ErrorResponse {
                error: "Invalid API key".to_string(),
            });
        }
    } else {
        return HttpResponse::Unauthorized().json(ErrorResponse {
            error: "Missing API key".to_string(),
        });
    }

    let projects: Vec<ProjectSummary> = config.projects
        .iter()
        .map(|(name, path)| ProjectSummary {
            name: name.clone(),
            path: path.to_string_lossy().into_owned(),
        })
        .collect();

    info!("Listed {} projects", projects.len());
    let response = ProjectListResponse { projects };
    info!("Response: {}", serde_json::to_string(&response).unwrap());
    HttpResponse::Ok().json(response)
}

pub async fn get_project_metadata(
    req: HttpRequest,
    project_name: web::Path<String>,
    data: web::Data<AppState>,
) -> impl Responder {
    let config = data.config.read().await;
    if let Some(api_key) = req.headers().get("X-API-Key") {
        if !validate_api_key(&config, api_key.to_str().unwrap_or("")).await {
            return HttpResponse::Unauthorized().json(ErrorResponse {
                error: "Invalid API key".to_string(),
            });
        }
    } else {
        return HttpResponse::Unauthorized().json(ErrorResponse {
            error: "Missing API key".to_string(),
        });
    }

    let project_name = project_name.into_inner();

    if let Some(project_path) = config.projects.get(&project_name) {
        debug!("Gathering metadata for project: {}", project_name);
        match gather_relevant_files(project_path.to_str().unwrap(), vec![], vec![]) {
            Ok(files) => {
                let file_paths: Vec<String> = files
                    .iter()
                    .map(|path| path.strip_prefix(project_path).unwrap().to_string_lossy().into_owned())
                    .collect();

                let metadata = ProjectMetadata {
                    name: project_name,
                    path: project_path.to_string_lossy().into_owned(),
                    files: file_paths,
                };

                info!("Successfully retrieved metadata for project: {}", metadata.name);
                info!("Response: {}", serde_json::to_string(&metadata).unwrap());
                HttpResponse::Ok().json(metadata)
            },
            Err(e) => {
                error!("Error gathering files for project {}: {}", project_name, e);
                HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "Failed to gather project metadata".to_string(),
                })
            }
        }
    } else {
        warn!("Project not found: {}", project_name);
        HttpResponse::NotFound().json(ErrorResponse {
            error: format!("Project '{}' not found", project_name),
        })
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
            return HttpResponse::Unauthorized().json(ErrorResponse {
                error: "Invalid API key".to_string(),
            });
        }
    } else {
        return HttpResponse::Unauthorized().json(ErrorResponse {
            error: "Missing API key".to_string(),
        });
    }

    let project_name = project_name.into_inner();

    if let Some(project_path) = config.projects.get(&project_name) {
        let base_path = project_path.clone();
        let files_to_process = if let Some(paths) = &contexter_req.paths {
            debug!("Running contexter on specific paths for project: {}", project_name);
            paths.iter().flat_map(|p| {
                let full_path = base_path.join(p);
                gather_relevant_files(full_path.to_str().unwrap(), vec![], vec![]).unwrap_or_default()
            }).collect()
        } else {
            debug!("Running contexter on entire project: {}", project_name);
            match gather_relevant_files(project_path.to_str().unwrap(), vec![], vec![]) {
                Ok(files) => files,
                Err(e) => {
                    error!("Error gathering files for project {}: {}", project_name, e);
                    return HttpResponse::InternalServerError().json(ErrorResponse {
                        error: "Failed to gather files".to_string(),
                    });
                }
            }
        };

        match concatenate_files(files_to_process) {
            Ok((content, processed_files)) => {
                info!("Successfully ran contexter on {} files for project: {}", processed_files.len(), project_name);
                let response = ProjectContentResponse { content };
                info!("Response: {}", serde_json::to_string(&response).unwrap());
                HttpResponse::Ok().json(response)
            },
            Err(e) => {
                error!("Error concatenating files for project {}: {}", project_name, e);
                HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "Failed to concatenate files".to_string(),
                })
            }
        }
    } else {
        warn!("Project not found: {}", project_name);
        HttpResponse::NotFound().json(ErrorResponse {
            error: format!("Project '{}' not found", project_name),
        })
    }
}

pub async fn run_server(config: Config) -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        config: Arc::new(RwLock::new(config)),
    });

    // Read configuration values before starting the server
    let listen_address = app_state.config.read().await.listen_address.clone();
    let port = app_state.config.read().await.port;

    info!("Starting server on {}:{}", listen_address, port);

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(
                web::scope("/api/v1")
                    .route("/projects", web::get().to(list_projects))
                    .route("/projects/{name}", web::get().to(get_project_metadata))
                    .route("/projects/{name}", web::post().to(run_contexter))
            )
    })
    .bind((listen_address, port))?
    .run()
    .await
}