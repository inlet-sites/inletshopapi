use actix_web::{HttpResponse, web, get};
use tokio::fs;
use std::path::Path;

use crate::app_error::AppError;

#[get("/documentation")]
pub async fn documentation_route() -> Result<HttpResponse, AppError> {
    match fs::read("./docs/redoc-static.html").await {
        Ok(f) => {
            Ok(HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(f))
        },
        Err(_) => Err(AppError::InternalError)
    }
}

#[get("/documents/{tail:.*}")]
pub async fn documents_route(path: web::Path<String>) -> Result<HttpResponse, AppError> {
    let home = std::env::var("HOME_DIR").expect("HOME_DIR not set");
    let base_dir = format!("{}srv/", home);
    let base = Path::new(&base_dir);
    let full_path = base.join(path.into_inner())
        .canonicalize()
        .map_err(|e| {
            eprintln!("{:?}", e);
            AppError::not_found("Invalid path")
        })?;

    if !full_path.starts_with(base) {
        return Err(AppError::not_found("Invalid path"));
    }

    match fs::read(&full_path).await {
        Ok(f) => Ok(HttpResponse::Ok().content_type(get_mime(&full_path)).body(f)),
        Err(_) => Err(AppError::not_found("File not found"))
    }
}

fn get_mime(path: &Path) -> String {
    match path.extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase().as_str() {
        "avif" => String::from("image/avif"),
        _ => String::from("application/octet-stream")
    }
}
