use axum::{
    extract::{ConnectInfo, State},
    http::StatusCode,
    Json,
};
use std::net::SocketAddr;
use std::sync::Arc;
use shared::{RegisterRequest, LoginRequest, AuthResponse, ApiError, UserProfile};
use crate::{AppState, auth, db::UserRepository};

/// Validate email format using a simple regex pattern
fn is_valid_email(email: &str) -> bool {
    // Basic email regex: local@domain.tld
    let email_regex = regex::Regex::new(
        r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$"
    ).unwrap();
    email_regex.is_match(email) && email.len() <= 254
}

/// Validate username: alphanumeric and underscore only, 3-30 chars
fn is_valid_username(username: &str) -> bool {
    let username_regex = regex::Regex::new(r"^[a-zA-Z0-9_]{3,30}$").unwrap();
    username_regex.is_match(username)
}

/// Validate password complexity: at least 8 chars, contains letter and digit
fn is_valid_password(password: &str) -> bool {
    password.len() >= 8
        && password.len() <= 128
        && password.chars().any(|c| c.is_ascii_alphabetic())
        && password.chars().any(|c| c.is_ascii_digit())
}

pub async fn register(
    State(state): State<Arc<AppState>>,
    Json(req): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, Json<ApiError>)> {
    // Validate username: alphanumeric and underscore, 3-30 chars
    if !is_valid_username(&req.username) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ApiError::validation_error(
                "Username must be 3-30 characters and contain only letters, numbers, and underscores"
            )),
        ));
    }

    // Validate password complexity
    if !is_valid_password(&req.password) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ApiError::validation_error(
                "Password must be at least 8 characters and contain both letters and numbers"
            )),
        ));
    }

    // Validate email format
    if !is_valid_email(&req.email) {
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
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, Json<ApiError>)> {
    let client_ip = addr.ip();

    // Check rate limit before processing
    if let Err(seconds_remaining) = state.login_rate_limiter.check(client_ip) {
        return Err((
            StatusCode::TOO_MANY_REQUESTS,
            Json(ApiError::new(
                "TOO_MANY_REQUESTS",
                format!("Too many login attempts. Please try again in {} seconds.", seconds_remaining),
            )),
        ));
    }

    // Find user by email
    let user = match UserRepository::find_by_email(&state.db.pool, &req.email).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            // Record failed attempt even for non-existent user (prevent enumeration)
            let _ = state.login_rate_limiter.record_failure(client_ip);
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(ApiError::unauthorized("Invalid email or password")),
            ));
        }
        Err(e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiError::internal_error(e.to_string())),
            ));
        }
    };

    // Verify password
    let is_valid = auth::verify_password(&req.password, &user.password_hash)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::internal_error(e.to_string()))))?;

    if !is_valid {
        // Record failed attempt
        if let Err(seconds_remaining) = state.login_rate_limiter.record_failure(client_ip) {
            return Err((
                StatusCode::TOO_MANY_REQUESTS,
                Json(ApiError::new(
                    "TOO_MANY_REQUESTS",
                    format!("Too many login attempts. Please try again in {} seconds.", seconds_remaining),
                )),
            ));
        }
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(ApiError::unauthorized("Invalid email or password")),
        ));
    }

    // Successful login - clear rate limit for this IP
    state.login_rate_limiter.clear(client_ip);

    // Generate token
    let token = auth::create_token(user.id, &user.email, &user.role, &state.jwt_secret)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::internal_error(e.to_string()))))?;

    Ok(Json(AuthResponse {
        token,
        user: UserProfile::from(user),
    }))
}
