use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use std::sync::Arc;
use shared::{RegisterRequest, LoginRequest, AuthResponse, ApiError, UserProfile};
use crate::{AppState, auth, db::UserRepository};

pub async fn register(
    State(state): State<Arc<AppState>>,
    Json(req): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, Json<ApiError>)> {
    // Validate input
    if req.username.len() < 3 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ApiError::validation_error("Username must be at least 3 characters")),
        ));
    }
    if req.password.len() < 6 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ApiError::validation_error("Password must be at least 6 characters")),
        ));
    }
    if !req.email.contains('@') {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ApiError::validation_error("Invalid email format")),
        ));
    }

    // Check if email exists
    if UserRepository::email_exists(&state.db.pool, &req.email)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::internal_error(e.to_string()))))?
    {
        return Err((
            StatusCode::CONFLICT,
            Json(ApiError::new("CONFLICT", "Email already registered")),
        ));
    }

    // Check if username exists
    if UserRepository::username_exists(&state.db.pool, &req.username)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::internal_error(e.to_string()))))?
    {
        return Err((
            StatusCode::CONFLICT,
            Json(ApiError::new("CONFLICT", "Username already taken")),
        ));
    }

    // Hash password
    let password_hash = auth::hash_password(&req.password)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::internal_error(e.to_string()))))?;

    // Create user
    let user = UserRepository::create(&state.db.pool, &req.username, &req.email, &password_hash)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::internal_error(e.to_string()))))?;

    // Generate token
    let token = auth::create_token(user.id, &user.email, &user.role, &state.jwt_secret)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::internal_error(e.to_string()))))?;

    Ok(Json(AuthResponse {
        token,
        user: UserProfile::from(user),
    }))
}

pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, Json<ApiError>)> {
    // Find user by email
    let user = UserRepository::find_by_email(&state.db.pool, &req.email)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::internal_error(e.to_string()))))?
        .ok_or_else(|| {
            (
                StatusCode::UNAUTHORIZED,
                Json(ApiError::unauthorized("Invalid email or password")),
            )
        })?;

    // Verify password
    let is_valid = auth::verify_password(&req.password, &user.password_hash)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::internal_error(e.to_string()))))?;

    if !is_valid {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(ApiError::unauthorized("Invalid email or password")),
        ));
    }

    // Generate token
    let token = auth::create_token(user.id, &user.email, &user.role, &state.jwt_secret)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::internal_error(e.to_string()))))?;

    Ok(Json(AuthResponse {
        token,
        user: UserProfile::from(user),
    }))
}
