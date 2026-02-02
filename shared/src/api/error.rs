use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiError {
    pub code: String,
    pub message: String,
}

impl ApiError {
    pub fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
        }
    }

    pub fn not_found(message: impl Into<String>) -> Self {
        Self::new("NOT_FOUND", message)
    }

    pub fn unauthorized(message: impl Into<String>) -> Self {
        Self::new("UNAUTHORIZED", message)
    }

    pub fn bad_request(message: impl Into<String>) -> Self {
        Self::new("BAD_REQUEST", message)
    }

    pub fn internal_error(message: impl Into<String>) -> Self {
        // In production, don't expose internal error details
        let is_production = std::env::var("RUST_ENV")
            .map(|v| v == "production" || v == "prod")
            .unwrap_or(false);

        if is_production {
            Self::new("INTERNAL_ERROR", "An internal error occurred. Please try again later.")
        } else {
            Self::new("INTERNAL_ERROR", message)
        }
    }

    pub fn validation_error(message: impl Into<String>) -> Self {
        Self::new("VALIDATION_ERROR", message)
    }
}

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Internal error: {0}")]
    Internal(String),

    #[error("Validation error: {0}")]
    Validation(String),
}

impl From<AppError> for ApiError {
    fn from(err: AppError) -> Self {
        match err {
            AppError::NotFound(msg) => ApiError::not_found(msg),
            AppError::Unauthorized(msg) => ApiError::unauthorized(msg),
            AppError::BadRequest(msg) => ApiError::bad_request(msg),
            AppError::Internal(msg) => ApiError::internal_error(msg),
            AppError::Validation(msg) => ApiError::validation_error(msg),
        }
    }
}
