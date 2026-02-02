use axum::{
    extract::{Path, State},
    http::{header, StatusCode},
    Json,
};
use std::sync::Arc;
use uuid::Uuid;
use shared::{CreateOrderRequest, OrderResponse, OrderListResponse, MessageResponse, ApiError, OrderStatus};
use crate::{AppState, auth, db::{CartRepository, OrderRepository, ProductRepository}};

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

pub async fn list_orders(
    State(state): State<Arc<AppState>>,
    headers: axum::http::HeaderMap,
) -> Result<Json<OrderListResponse>, (StatusCode, Json<ApiError>)> {
    let auth_header = headers.get(header::AUTHORIZATION).and_then(|v| v.to_str().ok());
    let user_id = get_user_id(&state, auth_header).await?;

    let orders = OrderRepository::list_by_user(&state.db.pool, user_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::internal_error(e.to_string()))))?;

    let total = orders.len() as u32;

    Ok(Json(OrderListResponse { orders, total }))
}

pub async fn create_order(
    State(state): State<Arc<AppState>>,
    headers: axum::http::HeaderMap,
    Json(_req): Json<CreateOrderRequest>,
) -> Result<Json<OrderResponse>, (StatusCode, Json<ApiError>)> {
    let auth_header = headers.get(header::AUTHORIZATION).and_then(|v| v.to_str().ok());
    let user_id = get_user_id(&state, auth_header).await?;

    // Get cart items first (outside transaction for read)
    let cart = CartRepository::get_cart(&state.db.pool, user_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::internal_error(e.to_string()))))?;

    if cart.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ApiError::bad_request("Cart is empty")),
        ));
    }

    // Use transaction for atomic stock check, update, order creation, and cart clear
    let order_with_items = OrderRepository::create_order_atomic(&state.db.pool, user_id, &cart.items)
        .await
        .map_err(|e| {
            let msg = e.to_string();
            if msg.contains("Insufficient stock") || msg.contains("not found") {
                (StatusCode::BAD_REQUEST, Json(ApiError::bad_request(msg)))
            } else {
                (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::internal_error(msg)))
            }
        })?;

    Ok(Json(OrderResponse { order: order_with_items }))
}

pub async fn get_order(
    State(state): State<Arc<AppState>>,
    headers: axum::http::HeaderMap,
    Path(id): Path<String>,
) -> Result<Json<OrderResponse>, (StatusCode, Json<ApiError>)> {
    let auth_header = headers.get(header::AUTHORIZATION).and_then(|v| v.to_str().ok());
    let user_id = get_user_id(&state, auth_header).await?;

    let id: Uuid = id.parse().map_err(|_| {
        (StatusCode::BAD_REQUEST, Json(ApiError::bad_request("Invalid order ID")))
    })?;

    let order_with_items = OrderRepository::get_by_id(&state.db.pool, id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::internal_error(e.to_string()))))?
        .ok_or_else(|| {
            (StatusCode::NOT_FOUND, Json(ApiError::not_found("Order not found")))
        })?;

    // Verify ownership
    if order_with_items.order.user_id != user_id {
        return Err((
            StatusCode::FORBIDDEN,
            Json(ApiError::new("FORBIDDEN", "You don't have access to this order")),
        ));
    }

    Ok(Json(OrderResponse { order: order_with_items }))
}

pub async fn cancel_order(
    State(state): State<Arc<AppState>>,
    headers: axum::http::HeaderMap,
    Path(id): Path<String>,
) -> Result<Json<MessageResponse>, (StatusCode, Json<ApiError>)> {
    let auth_header = headers.get(header::AUTHORIZATION).and_then(|v| v.to_str().ok());
    let user_id = get_user_id(&state, auth_header).await?;

    let id: Uuid = id.parse().map_err(|_| {
        (StatusCode::BAD_REQUEST, Json(ApiError::bad_request("Invalid order ID")))
    })?;

    let order_with_items = OrderRepository::get_by_id(&state.db.pool, id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::internal_error(e.to_string()))))?
        .ok_or_else(|| {
            (StatusCode::NOT_FOUND, Json(ApiError::not_found("Order not found")))
        })?;

    // Verify ownership
    if order_with_items.order.user_id != user_id {
        return Err((
            StatusCode::FORBIDDEN,
            Json(ApiError::new("FORBIDDEN", "You don't have access to this order")),
        ));
    }

    // Can only cancel pending orders
    if !order_with_items.order.can_cancel() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ApiError::bad_request("Only pending orders can be cancelled")),
        ));
    }

    // Update status
    OrderRepository::update_status(&state.db.pool, id, OrderStatus::Cancelled)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::internal_error(e.to_string()))))?;

    // Restore stock
    for item in &order_with_items.items {
        ProductRepository::update_stock(&state.db.pool, item.product_id, item.quantity)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::internal_error(e.to_string()))))?;
    }

    Ok(Json(MessageResponse {
        message: "Order cancelled successfully".to_string(),
    }))
}
