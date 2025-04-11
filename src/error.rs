use std::io;

use axum::{Error as AxumError, response::IntoResponse};

// TODO: add more custom error types
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid event")]
    InvalidEvent,
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("Server error: {0}")]
    Server(#[from] AxumError),
}

// TODO: consider returning json body for errors
impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::InvalidEvent =>
                (axum::http::StatusCode::BAD_REQUEST, self.to_string()).into_response(),
            Self::Io(e) =>
                (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
            Self::Server(e) =>
                (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        }
    }
}
