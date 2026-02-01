use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use std::sync::Arc;
use uuid::Uuid;
use shared::{ProductListParams, ProductListResponse, ProductResponse, CategoryListResponse, ApiError};
use crate::{AppState, db::ProductRepository};

pub async fn list_products(
    State(state): State<Arc<AppState>>,
    Query(params): Query<ProductListParams>,
) -> Result<Json<ProductListResponse>, (StatusCode, Json<ApiError>)> {
    let page = params.page.unwrap_or(1).max(1);
    let limit = params.limit.unwrap_or(12).min(100);

    let (products, total) = ProductRepository::list(
        &state.db.pool,
        page,
        limit,
        params.sort_by.as_deref(),
        params.sort_order.as_deref(),
    )
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::internal_error(e.to_string()))))?;

    let total_pages = (total as f64 / limit as f64).ceil() as u32;

    Ok(Json(ProductListResponse {
        products,
        total,
        page,
        limit,
        total_pages,
    }))
}

pub async fn get_product(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<ProductResponse>, (StatusCode, Json<ApiError>)> {
    let id: Uuid = id.parse().map_err(|_| {
        (StatusCode::BAD_REQUEST, Json(ApiError::bad_request("Invalid product ID")))
    })?;

    let product = ProductRepository::get_by_id(&state.db.pool, id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::internal_error(e.to_string()))))?
        .ok_or_else(|| {
            (StatusCode::NOT_FOUND, Json(ApiError::not_found("Product not found")))
        })?;

    Ok(Json(ProductResponse { product }))
}

pub async fn search_products(
    State(state): State<Arc<AppState>>,
    Query(params): Query<ProductListParams>,
) -> Result<Json<ProductListResponse>, (StatusCode, Json<ApiError>)> {
    let query = params.search.unwrap_or_default();
    let limit = params.limit.unwrap_or(20).min(100);

    let products = ProductRepository::search(&state.db.pool, &query, limit)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::internal_error(e.to_string()))))?;

    let total = products.len() as u32;

    Ok(Json(ProductListResponse {
        products,
        total,
        page: 1,
        limit,
        total_pages: 1,
    }))
}

pub async fn products_by_category(
    State(state): State<Arc<AppState>>,
    Path(category): Path<String>,
    Query(params): Query<ProductListParams>,
) -> Result<Json<ProductListResponse>, (StatusCode, Json<ApiError>)> {
    let page = params.page.unwrap_or(1).max(1);
    let limit = params.limit.unwrap_or(12).min(100);

    let (products, total) = ProductRepository::filter_by_category(&state.db.pool, &category, page, limit)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::internal_error(e.to_string()))))?;

    let total_pages = (total as f64 / limit as f64).ceil() as u32;

    Ok(Json(ProductListResponse {
        products,
        total,
        page,
        limit,
        total_pages,
    }))
}

pub async fn list_categories(
    State(state): State<Arc<AppState>>,
) -> Result<Json<CategoryListResponse>, (StatusCode, Json<ApiError>)> {
    let categories = ProductRepository::list_categories(&state.db.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::internal_error(e.to_string()))))?;

    Ok(Json(CategoryListResponse { categories }))
}
