use axum::{
    extract::{Request, State},
    http::{header, StatusCode},
    middleware::Next,
    response::Response,
    Json,
};
use std::sync::Arc;
use shared::ApiError;
use crate::AppState;
use super::jwt::{verify_token, Claims};

pub async fn auth_middleware(
    State(state): State<Arc<AppState>>,
    mut request: Request,
    next: Next,
) -> Result<Response, (StatusCode, Json<ApiError>)> {
    let auth_header = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok());

    let token = match auth_header {
        Some(header) if header.starts_with("Bearer ") => &header[7..],
        _ => {
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(ApiError::unauthorized("Missing or invalid authorization header")),
            ));
        }
    };

    let claims = verify_token(token, &state.jwt_secret).map_err(|_| {
        (
            StatusCode::UNAUTHORIZED,
            Json(ApiError::unauthorized("Invalid or expired token")),
        )
    })?;

    // Store claims in request extensions for later use
    request.extensions_mut().insert(claims);

    Ok(next.run(request).await)
}

// Helper extractor to get current user from request
pub struct CurrentUser(pub Claims);

#[axum::async_trait]
impl<S> axum::extract::FromRequestParts<S> for CurrentUser
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, Json<ApiError>);

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<Claims>()
            .cloned()
            .map(CurrentUser)
            .ok_or_else(|| {
                (
                    StatusCode::UNAUTHORIZED,
                    Json(ApiError::unauthorized("Not authenticated")),
                )
            })
    }
}
