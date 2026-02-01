use axum::{
    extract::{Path, State},
    http::{header, StatusCode},
    Json,
};
use std::sync::Arc;
use uuid::Uuid;
use shared::{AddToCartRequest, UpdateCartItemRequest, CartResponse, MessageResponse, ApiError};
use crate::{AppState, auth, db::{CartRepository, ProductRepository}};

// Helper to extract user from token
async fn get_user_id(
    state: &AppState,
    auth_header: Option<&str>,
) -> Result<Uuid, (StatusCode, Json<ApiError>)> {
    let token = auth_header
        .and_then(|h| h.strip_prefix("Bearer "))
        .ok_or_else(|| {
            (StatusCode::UNAUTHORIZED, Json(ApiError::unauthorized("Missing authorization header")))
        })?;

    let claims = auth::verify_token(token, &state.jwt_secret)
        .map_err(|_| {
            (StatusCode::UNAUTHORIZED, Json(ApiError::unauthorized("Invalid or expired token")))
        })?;

    Ok(claims.sub)
}

pub async fn get_cart(
    State(state): State<Arc<AppState>>,
    headers: axum::http::HeaderMap,
) -> Result<Json<CartResponse>, (StatusCode, Json<ApiError>)> {
    let auth_header = headers.get(header::AUTHORIZATION).and_then(|v| v.to_str().ok());
    let user_id = get_user_id(&state, auth_header).await?;

    let cart = CartRepository::get_cart(&state.db.pool, user_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::internal_error(e.to_string()))))?;

    Ok(Json(CartResponse { cart }))
}

pub async fn add_to_cart(
    State(state): State<Arc<AppState>>,
    headers: axum::http::HeaderMap,
    Json(req): Json<AddToCartRequest>,
) -> Result<Json<CartResponse>, (StatusCode, Json<ApiError>)> {
    let auth_header = headers.get(header::AUTHORIZATION).and_then(|v| v.to_str().ok());
    let user_id = get_user_id(&state, auth_header).await?;

    // Validate quantity
    if req.quantity <= 0 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ApiError::validation_error("Quantity must be positive")),
        ));
    }

    // Check product exists and has stock
    let product = ProductRepository::get_by_id(&state.db.pool, req.product_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::internal_error(e.to_string()))))?
        .ok_or_else(|| {
            (StatusCode::NOT_FOUND, Json(ApiError::not_found("Product not found")))
        })?;

    if !product.is_available() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ApiError::bad_request("Product is not available")),
        ));
    }

    if product.stock < req.quantity {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ApiError::bad_request("Insufficient stock")),
        ));
    }

    // Add to cart
    CartRepository::add_item(&state.db.pool, user_id, req.product_id, req.quantity)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::internal_error(e.to_string()))))?;

    // Return updated cart
    let cart = CartRepository::get_cart(&state.db.pool, user_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::internal_error(e.to_string()))))?;

    Ok(Json(CartResponse { cart }))
}

pub async fn update_cart_item(
    State(state): State<Arc<AppState>>,
    headers: axum::http::HeaderMap,
    Path(product_id): Path<String>,
    Json(req): Json<UpdateCartItemRequest>,
) -> Result<Json<CartResponse>, (StatusCode, Json<ApiError>)> {
    let auth_header = headers.get(header::AUTHORIZATION).and_then(|v| v.to_str().ok());
    let user_id = get_user_id(&state, auth_header).await?;

    let product_id: Uuid = product_id.parse().map_err(|_| {
        (StatusCode::BAD_REQUEST, Json(ApiError::bad_request("Invalid product ID")))
    })?;

    if req.quantity <= 0 {
        // Remove item if quantity is 0 or negative
        CartRepository::remove_item(&state.db.pool, user_id, product_id)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::internal_error(e.to_string()))))?;
    } else {
        // Check stock
        let product = ProductRepository::get_by_id(&state.db.pool, product_id)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::internal_error(e.to_string()))))?
            .ok_or_else(|| {
                (StatusCode::NOT_FOUND, Json(ApiError::not_found("Product not found")))
            })?;

        if product.stock < req.quantity {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(ApiError::bad_request("Insufficient stock")),
            ));
        }

        CartRepository::update_item_quantity(&state.db.pool, user_id, product_id, req.quantity)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::internal_error(e.to_string()))))?;
    }

    // Return updated cart
    let cart = CartRepository::get_cart(&state.db.pool, user_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::internal_error(e.to_string()))))?;

    Ok(Json(CartResponse { cart }))
}

pub async fn remove_from_cart(
    State(state): State<Arc<AppState>>,
    headers: axum::http::HeaderMap,
    Path(product_id): Path<String>,
) -> Result<Json<MessageResponse>, (StatusCode, Json<ApiError>)> {
    let auth_header = headers.get(header::AUTHORIZATION).and_then(|v| v.to_str().ok());
    let user_id = get_user_id(&state, auth_header).await?;

    let product_id: Uuid = product_id.parse().map_err(|_| {
        (StatusCode::BAD_REQUEST, Json(ApiError::bad_request("Invalid product ID")))
    })?;

    let removed = CartRepository::remove_item(&state.db.pool, user_id, product_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::internal_error(e.to_string()))))?;

    if !removed {
        return Err((
            StatusCode::NOT_FOUND,
            Json(ApiError::not_found("Item not in cart")),
        ));
    }

    Ok(Json(MessageResponse {
        message: "Item removed from cart".to_string(),
    }))
}
