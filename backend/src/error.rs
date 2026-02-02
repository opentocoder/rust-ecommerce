use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use shared::ApiError;

pub struct AppError(pub anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // Log the actual error for debugging
        tracing::error!("Internal error: {}", self.0);

        // Return sanitized error to client
        let error = ApiError::internal_error(self.0.to_string());
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error)).into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

pub type Result<T> = std::result::Result<T, AppError>;

// Helper trait for converting errors with status codes
pub trait IntoApiError {
    fn not_found(message: impl Into<String>) -> (StatusCode, Json<ApiError>) {
        (StatusCode::NOT_FOUND, Json(ApiError::not_found(message)))
    }

    fn unauthorized(message: impl Into<String>) -> (StatusCode, Json<ApiError>) {
        (StatusCode::UNAUTHORIZED, Json(ApiError::unauthorized(message)))
    }

    fn bad_request(message: impl Into<String>) -> (StatusCode, Json<ApiError>) {
        (StatusCode::BAD_REQUEST, Json(ApiError::bad_request(message)))
    }

    fn internal_error(message: impl Into<String>) -> (StatusCode, Json<ApiError>) {
        (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::internal_error(message)))
    }
}

impl IntoApiError for ApiError {}
