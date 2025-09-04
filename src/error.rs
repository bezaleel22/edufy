use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Authentication error: {0}")]
    Auth(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("JWT error: {0}")]
    Jwt(#[from] jsonwebtoken::errors::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Scheduler error: {0}")]
    Scheduler(#[from] tokio_cron_scheduler::JobSchedulerError),

    #[error("Bcrypt error: {0}")]
    Bcrypt(#[from] bcrypt::BcryptError),

    #[error("Configuration error: {0}")]
    Config(#[from] config::ConfigError),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Migration error: {0}")]
    Migration(#[from] sqlx::migrate::MigrateError),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Internal server error")]
    Internal(String),

    #[error("Anyhow error: {0}")]
    Anyhow(#[from] anyhow::Error),

    #[error("HTTP request error: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("Multipart error: {0}")]
    Multipart(#[from] axum::extract::multipart::MultipartError),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match &self {
            AppError::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error"),
            AppError::Auth(_) => (StatusCode::UNAUTHORIZED, "Authentication failed"),
            AppError::Validation(msg) => (StatusCode::BAD_REQUEST, msg.as_str()),
            AppError::Jwt(_) => (StatusCode::UNAUTHORIZED, "Invalid token"),
            AppError::Bcrypt(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Password hashing error"),
            AppError::Config(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Configuration error"),
            AppError::Io(_) => (StatusCode::INTERNAL_SERVER_ERROR, "IO error"),
            AppError::Migration(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Migration error"),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg.as_str()),
            AppError::Conflict(msg) => (StatusCode::CONFLICT, msg.as_str()),
            AppError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.as_str()),
            AppError::Anyhow(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Internal error"),
            AppError::Reqwest(_) => (StatusCode::INTERNAL_SERVER_ERROR, "HTTP request error"),
            AppError::Multipart(_) => (StatusCode::BAD_REQUEST, "Multipart form error"),
            AppError::Json(_) => (StatusCode::INTERNAL_SERVER_ERROR, "JSON processing error"),
            AppError::Scheduler(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Scheduler error"),
        };

        let body = Json(json!({
            "error": {
                "message": error_message,
                "type": self.to_string()
            }
        }));

        (status, body).into_response()
    }
}

// Type alias for results
pub type AppResult<T> = Result<T, AppError>;
