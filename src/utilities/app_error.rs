use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use tokio::task::JoinError;

#[derive(Debug)]
pub struct AppError {
    pub code: StatusCode,
    pub message: String,
}

#[derive(Serialize, Deserialize)]
struct ErrorResponse {
    error: String,
}

impl AppError {
    pub fn new(code: StatusCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (
            self.code,
            Json(ErrorResponse {
                error: self.message.clone(),
            }),
        )
            .into_response()
    }
}

// to map from reqwest error to AppError
impl From<reqwest::Error> for AppError {
    fn from(value: reqwest::Error) -> Self {
        eprintln!("Reqwest error: {}", value);
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Reqwest error: {}", value),
        )
    }
}

// to map from json web token error to AppError
impl From<jsonwebtoken::errors::Error> for AppError {
    fn from(value: jsonwebtoken::errors::Error) -> Self {
        eprintln!("JsonWebToken error: {}", value);
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("JsonWebToken error: {}", value),
        )
    }
}

// to map from sqlx error to AppError
impl From<sqlx::Error> for AppError {
    fn from(value: sqlx::Error) -> Self {
        eprintln!("SQLx error: {}", value);
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("SQLx error: {}", value),
        )
    }
}

// to map from tokio async task's JoinError to AppError
impl From<JoinError> for AppError {
    fn from(value: JoinError) -> Self {
        eprintln!("Tokio task JoinError: {}", value);
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Tokio task JoinError: {}", value),
        )
    }
}
