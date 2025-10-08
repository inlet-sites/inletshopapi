use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use thiserror::Error;
use serde::Serialize;

#[derive(Serialize)]
struct ErrorBody {
    error: ErrorInfo
}

#[derive(Serialize)]
struct ErrorInfo {
    code: u16,
    message: String
}

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Internal Server Error")]
    InternalError,

    #[error("{0}")]
    InvalidInput(String),

    #[error("Unauthorized")]
    Auth,

    #[error("{0}")]
    NotFound(String),

    #[error("{0}")]
    Forbidden(String),

    #[error("Internal Server Error")]
    Database(#[from] mongodb::error::Error)
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::InvalidInput(_) => StatusCode::BAD_REQUEST,
            AppError::Auth => StatusCode::UNAUTHORIZED,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::Forbidden(_) => StatusCode::FORBIDDEN,
            AppError::Database(_) => StatusCode::INTERNAL_SERVER_ERROR
        }
    }

    fn error_response(&self) -> HttpResponse {
        let body = ErrorBody {
            error: ErrorInfo {
                code: self.status_code().as_u16(),
                message: self.to_string()
            }
        };

        HttpResponse::build(self.status_code()).json(body)
    }
}

impl AppError {
    pub fn invalid_input(msg: &str) -> Self {
        AppError::InvalidInput(msg.to_owned())
    }

    pub fn not_found(msg: &str) -> Self {
        AppError::NotFound(msg.to_owned())
    }

    pub fn forbidden(msg: &str) -> Self {
        AppError::Forbidden(msg.to_owned())
    }
}
