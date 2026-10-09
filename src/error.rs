use axum::{
    response::{IntoResponse, Response},
    http::StatusCode,
};
use log::info;

#[allow(dead_code)]
pub enum AppError {
    NotFound,
    BadRequest(String),
    Internal(anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        info!("Into response");
        match self {
            AppError::NotFound => (StatusCode::NOT_FOUND, "JSON file not found")
                .into_response(),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg)
                .into_response(),
            AppError::Internal(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Internal server error: {}", err),
            )
                .into_response(),
        }
    }
}

// Map file-not-found errors to 404, other IO errors to 500
impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        if err.kind() == std::io::ErrorKind::NotFound {
            info!("IO error Not found");
            AppError::NotFound
        } else {
            info!("App erro?");
            AppError::Internal(err.into())
        }
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        info!("Serde error?");
        AppError::Internal(err.into())
    }
}
