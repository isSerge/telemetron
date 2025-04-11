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

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        (axum::http::StatusCode::BAD_REQUEST, self.to_string()).into_response()
    }
}
