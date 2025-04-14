use std::io;

use axum::{Error as AxumError, response::IntoResponse};

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
    InternalServerError(String),
}

const INTERNAL_ERROR_MESSAGE: &str = "Internal server error";

// TODO: consider returning json body for errors
impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::InternalServerError(msg) => {
                tracing::error!("Internal server error: {}", msg);
                (axum::http::StatusCode::INTERNAL_SERVER_ERROR, INTERNAL_ERROR_MESSAGE)
                    .into_response()
            }
            Self::InvalidEvent(e) => {
                tracing::warn!("Invalid event rejected: {}", e);
                (axum::http::StatusCode::BAD_REQUEST, e.to_string()).into_response()
            }
            Self::Io(e) => {
                tracing::error!("IO error: {}", e);
                (axum::http::StatusCode::INTERNAL_SERVER_ERROR, INTERNAL_ERROR_MESSAGE)
                    .into_response()
            }
            Self::Server(e) => {
                tracing::error!("Server error: {}", e);
                (axum::http::StatusCode::INTERNAL_SERVER_ERROR, INTERNAL_ERROR_MESSAGE)
                    .into_response()
            }
        }
    }
}
