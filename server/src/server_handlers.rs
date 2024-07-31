use crate::contexter::{concatenate_files, gather_relevant_files};
use crate::server::{
    AppState, ErrorResponse, ProjectContentResponse, ProjectListResponse, ProjectMetadata,
    ProjectSummary,
};
use crate::utils::validate_api_key;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use log::{debug, error, info, warn};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ContexterRequest {
    pub paths: Option<Vec<String>>,
}

pub async fn list_projects(req: HttpRequest, data: web::Data<AppState>) -> impl Responder {
    let config = data.config.read().await;
    if !validate_api_key(&req, &config).await {
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

pub async fn get_project_metadata(
    req: HttpRequest,
    project_name: web::Path<String>,
    data: web::Data<AppState>,
) -> impl Responder {
    let config = data.config.read().await;
    if !validate_api_key(&req, &config).await {
        return HttpResponse::Unauthorized().json(ErrorResponse {
            error: "Invalid or missing API key".to_string(),
        });
    }

    let project_name = project_name.into_inner();

    if let Some(project_path) = config.projects.get(&project_name) {
        debug!("Gathering metadata for project: {}", project_name);
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
    contexter_req: web::Json<Option<ContexterRequest>>,
    data: web::Data<AppState>,
) -> impl Responder {
    let config = data.config.read().await;
    if !validate_api_key(&req, &config).await {
        return HttpResponse::Unauthorized().json(ErrorResponse {
            error: "Invalid or missing API key".to_string(),
        });
    }

    let project_name = project_name.into_inner();

    if let Some(project_path) = config.projects.get(&project_name) {
        let base_path = project_path.clone();
        let files_to_process = if let Some(ContexterRequest { paths }) = contexter_req.into_inner()
        {
            if let Some(paths) = paths {
                debug!(
                    "Running contexter on specific paths for project: {}",
                    project_name
                );
                paths
                    .iter()
                    .flat_map(|p| {
                        let full_path = base_path.join(p);
                        gather_relevant_files(full_path.to_str().unwrap(), vec![], vec![])
                            .unwrap_or_default()
                    })
                    .collect()
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
            }
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
                info!(
                    "Successfully ran contexter on {} files for project: {}",
                    processed_files.len(),
                    project_name
                );
                let response = ProjectContentResponse { content };
                HttpResponse::Ok().json(response)
            }
            Err(e) => {
                error!(
                    "Error concatenating files for project {}: {}",
                    project_name, e
                );
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
