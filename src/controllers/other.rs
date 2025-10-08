use actix_web::{HttpResponse, get};
use std::fs;

use crate::app_error::AppError;

#[get("/documentation")]
pub async fn documentation_route() -> Result<HttpResponse, AppError> {
    match fs::read_to_string("./docs/redoc-static.html") {
        Ok(f) => Ok(HttpResponse::Ok().body(f)),
        Err(_) => Err(AppError::InternalError)
    }
}
