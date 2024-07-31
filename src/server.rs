use actix_web::{web, App, HttpServer, HttpResponse, Responder, HttpRequest};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use std::fs::read_to_string;
use crate::gather_relevant_files;

#[derive(Deserialize)]
pub struct FileRequest {
    pub directory: String,
    pub extensions: Vec<String>,
    pub exclude_patterns: Vec<String>,
}

#[derive(Deserialize)]
pub struct ReadFileRequest {
    pub file_path: String,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

#[derive(Serialize)]
pub struct FileResponse {
    pub files: Vec<String>,
}

#[derive(Serialize)]
pub struct FileContentResponse {
    pub content: String,
}

#[derive(Clone)]
pub struct ApiKey {
    pub key: String,
}

async fn authorize(api_key: Arc<RwLock<ApiKey>>, header_key: Option<String>) -> bool {
    if let Some(header_key) = header_key {
        if header_key == api_key.read().await.key {
            return true;
        }
    }
    false
}

async fn handle_get_files(
    api_key: web::Data<Arc<RwLock<ApiKey>>>,
    req: web::Json<FileRequest>,
    headers: HttpRequest,
) -> impl Responder {
    let header_key = headers.headers().get("X-API-KEY").and_then(|v| v.to_str().ok());

    if !authorize(api_key.get_ref().clone(), header_key.map(|s| s.to_string())).await {
        return HttpResponse::Unauthorized().json(ErrorResponse {
            error: "Unauthorized".to_string(),
        });
    }

    let files = gather_relevant_files(
        &req.directory,
        req.extensions.iter().map(String::as_str).collect(),
        req.exclude_patterns.iter().map(String::as_str).collect(),
    )
    .unwrap_or_else(|_| Vec::new());

    HttpResponse::Ok().json(FileResponse {
        files: files.into_iter().map(|p| p.to_string_lossy().to_string()).collect(),
    })
}

async fn handle_read_file(
    api_key: web::Data<Arc<RwLock<ApiKey>>>,
    req: web::Json<ReadFileRequest>,
    headers: HttpRequest,
) -> impl Responder {
    let header_key = headers.headers().get("X-API-KEY").and_then(|v| v.to_str().ok());

    if !authorize(api_key.get_ref().clone(), header_key.map(|s| s.to_string())).await {
        return HttpResponse::Unauthorized().json(ErrorResponse {
            error: "Unauthorized".to_string(),
        });
    }

    match read_to_string(&req.file_path) {
        Ok(content) => HttpResponse::Ok().json(FileContentResponse { content }),
        Err(_) => HttpResponse::NotFound().json(ErrorResponse {
            error: "File not found".to_string(),
        }),
    }
}

pub async fn run_server(api_key: String) -> std::io::Result<()> {
    let api_key = Arc::new(RwLock::new(ApiKey { key: api_key }));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(api_key.clone()))
            .service(
                web::resource("/get-files")
                    .route(web::post().to(handle_get_files)),
            )
            .service(
                web::resource("/read-file")
                    .route(web::post().to(handle_read_file)),
            )
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}
