use std::io;

use axum::{Error as AxumError, Json, response::IntoResponse};

use crate::event::EventValidationError;

// TODO: add more custom error types
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid event")]
    InvalidEvent(#[from] EventValidationError),
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("Server error: {0}")]
    Server(#[from] AxumError),
    #[error("Internal server error: {0}")]
    Internal(String),
    #[error("Not found")]
    NotFound(String),
}

const INTERNAL_ERROR_MESSAGE: &str = "Internal server error";

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            Self::Internal(msg) => {
                tracing::error!("Internal server error: {}", msg);
                (axum::http::StatusCode::INTERNAL_SERVER_ERROR, INTERNAL_ERROR_MESSAGE.to_string())
            }
            Self::InvalidEvent(e) => {
                tracing::warn!("Invalid event rejected: {}", e);
                (axum::http::StatusCode::BAD_REQUEST, e.to_string())
            }
            Self::Io(e) => {
                tracing::error!("IO error: {}", e);
                (axum::http::StatusCode::INTERNAL_SERVER_ERROR, INTERNAL_ERROR_MESSAGE.to_string())
            }
            Self::Server(e) => {
                tracing::error!("Server error: {}", e);
                (axum::http::StatusCode::INTERNAL_SERVER_ERROR, INTERNAL_ERROR_MESSAGE.to_string())
            }
            Self::NotFound(msg) => {
                tracing::warn!("Not found: {}", msg);
                (axum::http::StatusCode::NOT_FOUND, msg)
            }
        };

        let body = Json(serde_json::json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}
