use serde::{Deserialize, Serialize};
use uuid::Uuid;

// Auth requests
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

// Product requests
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProductListParams {
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub category: Option<String>,
    pub search: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
}

// Cart requests
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddToCartRequest {
    pub product_id: Uuid,
    pub quantity: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCartItemRequest {
    pub quantity: i32,
}

// Order requests
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateOrderRequest {
    pub shipping_address: Option<String>,
}

// Admin product requests
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProductRequest {
    pub name: String,
    pub description: String,
    pub price: f64,
    pub stock: i32,
    pub category: String,
    pub image_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateProductRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub price: Option<f64>,
    pub stock: Option<i32>,
    pub category: Option<String>,
    pub image_url: Option<String>,
    pub is_active: Option<bool>,
}
