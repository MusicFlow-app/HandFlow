use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use derive_more::Display;
use serde_json::json;
use std::{error::Error as StdError, io};

#[derive(Debug, Display)]
pub enum AppError {
    #[display(fmt = "Database error: {}", _0)]
    Database(sqlx::Error),
    #[display(fmt = "Not found: {}", _0)]
    NotFound(String),
    #[display(fmt = "Parse error: {}", _0)]
    Parse(String),
    #[display(fmt = "Upload error: {}", _0)]
    Upload(String),
    #[display(fmt = "IO error: {}", _0)]
    Io(io::Error)
}

impl StdError for AppError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            AppError::Database(e) => Some(e),
            AppError::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        let (status, error_message) = match self {
            AppError::Database(e) => {
                log::error!("Database error: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string())
            }
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg.to_string()),
            AppError::Parse(e) => {
                log::error!("Parse error: {}", e);
                (StatusCode::BAD_REQUEST, "Invalid request data".to_string())
            }
            AppError::Upload(msg) => (StatusCode::BAD_REQUEST, msg.to_string()),
            AppError::Io(e) => {
                log::error!("IO error: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string())
            }
        };

        HttpResponse::build(status)
            .json(json!({ "error": error_message }))
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            AppError::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::Parse(_) => StatusCode::BAD_REQUEST,
            AppError::Upload(_) => StatusCode::BAD_REQUEST,
            AppError::Io(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => AppError::NotFound("Resource not found".to_string()),
            _ => AppError::Database(err),
        }
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::Parse(err.to_string())
    }
}

impl From<io::Error> for AppError {
    fn from(err: io::Error) -> Self {
        AppError::Io(err)
    }
}
