use crate::contexter::{concatenate_files, gather_relevant_files};
use crate::repo_mapper::RepositoryMapper;
use crate::server::{
    AppState, ErrorResponse, ProjectContentResponse, ProjectListResponse, ProjectMetadata,
    ProjectSummary, RepositoryAnalysisResponse, RepositoryMapResponse,
};
use crate::utils::validate_api_key;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use log::{debug, error, info, warn};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ContexterRequest {
    pub paths: Option<Vec<String>>,
}

#[allow(clippy::future_not_send)]
pub async fn list_projects(req: HttpRequest, data: web::Data<AppState>) -> impl Responder {
    // Validate API key before await to avoid Send issues
    let config_guard = data.config.read().await;
    let is_valid = validate_api_key(&req, &config_guard);
    let config = config_guard.clone();
    drop(req); // Drop req to make future Send
    drop(config_guard); // Drop the guard

    if !is_valid {
        return HttpResponse::Unauthorized().json(ErrorResponse {
            error: "Invalid or missing API key".to_string(),
        });
    }

    let projects: Vec<ProjectSummary> = config
        .projects
        .iter()
        .map(|(name, path)| ProjectSummary {
            name: name.clone(),
            path: path.to_string_lossy().into_owned(),
        })
        .collect();

    info!("Listed {} projects", projects.len());
    let response = ProjectListResponse { projects };
    HttpResponse::Ok().json(response)
}

#[allow(clippy::future_not_send)]
pub async fn get_project_metadata(
    req: HttpRequest,
    project_name: web::Path<String>,
    data: web::Data<AppState>,
) -> impl Responder {
    let config = data.config.read().await;
    let is_valid = validate_api_key(&req, &config);
    drop(req); // Drop req to make future Send

    if !is_valid {
        return HttpResponse::Unauthorized().json(ErrorResponse {
            error: "Invalid or missing API key".to_string(),
        });
    }

    let project_name = project_name.into_inner();

    if let Some(project_path) = config.projects.get(&project_name) {
        debug!("Gathering metadata for project: {project_name}");
        match gather_relevant_files(
            project_path
                .to_str()
                .expect("Project path should be valid UTF-8"),
            &[],
            vec![],
        ) {
            Ok(files) => {
                let file_paths: Vec<String> = files
                    .iter()
                    .map(|path| {
                        path.strip_prefix(project_path)
                            .expect("Path should be under project directory")
                            .to_string_lossy()
                            .into_owned()
                    })
                    .collect();

                let metadata = ProjectMetadata {
                    name: project_name,
                    path: project_path.to_string_lossy().into_owned(),
                    files: file_paths,
                };

                info!(
                    "Successfully retrieved metadata for project: {}",
                    metadata.name
                );
                HttpResponse::Ok().json(metadata)
            }
            Err(e) => {
                error!("Error gathering files for project {project_name}: {e}");
                HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "Failed to gather project metadata".to_string(),
                })
            }
        }
    } else {
        warn!("Project not found: {project_name}");
        HttpResponse::NotFound().json(ErrorResponse {
            error: format!("Project '{project_name}' not found"),
        })
    }
}

#[allow(clippy::future_not_send)]
pub async fn run_contexter(
    req: HttpRequest,
    project_name: web::Path<String>,
    contexter_req: web::Json<Option<ContexterRequest>>,
    data: web::Data<AppState>,
) -> impl Responder {
    let config = data.config.read().await;
    let is_valid = validate_api_key(&req, &config);
    drop(req); // Drop req to make future Send

    if !is_valid {
        return HttpResponse::Unauthorized().json(ErrorResponse {
            error: "Invalid or missing API key".to_string(),
        });
    }

    let project_name = project_name.into_inner();

    if let Some(project_path) = config.projects.get(&project_name) {
        let base_path = project_path.clone();
        let files_to_process =
            if let Some(ContexterRequest { paths: Some(paths) }) = contexter_req.into_inner() {
                debug!("Running contexter on specific paths for project: {project_name}");
                paths
                    .iter()
                    .flat_map(|p| {
                        let full_path = base_path.join(p);
                        gather_relevant_files(
                            full_path.to_str().expect("Path should be valid UTF-8"),
                            &[],
                            vec![],
                        )
                        .unwrap_or_default()
                    })
                    .collect()
            } else {
                debug!("Running contexter on entire project: {project_name}");
                match gather_relevant_files(
                    project_path
                        .to_str()
                        .expect("Project path should be valid UTF-8"),
                    &[],
                    vec![],
                ) {
                    Ok(files) => files,
                    Err(e) => {
                        error!("Error gathering files for project {project_name}: {e}");
                        return HttpResponse::InternalServerError().json(ErrorResponse {
                            error: "Failed to gather files".to_string(),
                        });
                    }
                }
            };

        match concatenate_files(files_to_process) {
            Ok((content, processed_files)) => {
                info!(
                    "Successfully ran contexter on {} files for project: {}",
                    processed_files.len(),
                    project_name
                );
                let response = ProjectContentResponse { content };
                HttpResponse::Ok().json(response)
            }
            Err(e) => {
                error!("Error concatenating files for project {project_name}: {e}");
                HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "Failed to concatenate files".to_string(),
                })
            }
        }
    } else {
        warn!("Project not found: {project_name}");
        HttpResponse::NotFound().json(ErrorResponse {
            error: format!("Project '{project_name}' not found"),
        })
    }
}

// Repository mapping endpoints

#[allow(clippy::future_not_send)]
pub async fn analyze_repository(
    req: HttpRequest,
    project_name: web::Path<String>,
    data: web::Data<AppState>,
) -> impl Responder {
    let config = data.config.read().await;
    let is_valid = validate_api_key(&req, &config);
    drop(req); // Drop req to make future Send

    if !is_valid {
        return HttpResponse::Unauthorized().json(ErrorResponse {
            error: "Invalid or missing API key".to_string(),
        });
    }

    let project_name = project_name.into_inner();

    if let Some(project_path) = config.projects.get(&project_name) {
        debug!("Analyzing repository structure for project: {project_name}");

        let mut mapper = RepositoryMapper::new();
        match mapper.analyze_repository(project_path) {
            Ok(()) => {
                info!("Successfully analyzed repository: {project_name}");
                let response = RepositoryAnalysisResponse {
                    project_name: project_name.clone(),
                    total_components: mapper.insights.total_components,
                    entry_points: mapper.insights.entry_points.clone(),
                    dependency_cycles: mapper.graph.cycles.len(),
                    most_connected_components: mapper.insights.most_connected_components.clone(),
                    topological_order: mapper.topological_order.clone(),
                };
                HttpResponse::Ok().json(response)
            }
            Err(e) => {
                error!("Error analyzing repository {project_name}: {e}");
                HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "Failed to analyze repository".to_string(),
                })
            }
        }
    } else {
        warn!("Project not found: {project_name}");
        HttpResponse::NotFound().json(ErrorResponse {
            error: format!("Project '{project_name}' not found"),
        })
    }
}

#[allow(clippy::future_not_send)]
pub async fn get_repository_map(
    req: HttpRequest,
    project_name: web::Path<String>,
    data: web::Data<AppState>,
) -> impl Responder {
    let config = data.config.read().await;
    let is_valid = validate_api_key(&req, &config);
    drop(req); // Drop req to make future Send

    if !is_valid {
        return HttpResponse::Unauthorized().json(ErrorResponse {
            error: "Invalid or missing API key".to_string(),
        });
    }

    let project_name = project_name.into_inner();

    if let Some(project_path) = config.projects.get(&project_name) {
        debug!("Generating repository map for project: {project_name}");

        let mut mapper = RepositoryMapper::new();
        match mapper.analyze_repository(project_path) {
            Ok(()) => {
                let map = mapper.generate_repository_map();
                info!("Successfully generated repository map for: {project_name}");
                let response = RepositoryMapResponse {
                    project_name: project_name.clone(),
                    map,
                    insights: mapper.insights,
                };
                HttpResponse::Ok().json(response)
            }
            Err(e) => {
                error!("Error generating repository map for {project_name}: {e}");
                HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "Failed to generate repository map".to_string(),
                })
            }
        }
    } else {
        warn!("Project not found: {project_name}");
        HttpResponse::NotFound().json(ErrorResponse {
            error: format!("Project '{project_name}' not found"),
        })
    }
}
