use actix_web::{http::StatusCode, HttpResponse};
use thiserror::Error;

#[derive(Serialize)]
struct ErrorBody {
    error: ErrorInfo
}

#[derive(Serialize)]
struct ErrorInfo {
    code: u16,
    message: String
}

#[derive(Error)]
pub enum AppError {
    #[error("Internal Server Error")]
    InternalError
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::InternalError => StatusCode::INTERNAL_SERVER_ERROR
        }
    }

    fn error_response(&self) -> HttpResponse {
        let body = ErrorBody {
            error: ErrorInfo {
                code: self.status_code().as_u16(),
                message: self.to_string()
            }
        };

        HttpResponse::build(self.status_code().json(body))
    }
}
