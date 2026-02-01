use serde::{Deserialize, Serialize};
use crate::models::{Product, UserProfile, Cart, Order, OrderWithItems};

// Auth responses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserProfile,
}

// Product responses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductListResponse {
    pub products: Vec<Product>,
    pub total: u32,
    pub page: u32,
    pub limit: u32,
    pub total_pages: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductResponse {
    pub product: Product,
}

// Cart responses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CartResponse {
    pub cart: Cart,
}

// Order responses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderResponse {
    pub order: OrderWithItems,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderListResponse {
    pub orders: Vec<Order>,
    pub total: u32,
}

// Generic responses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageResponse {
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryListResponse {
    pub categories: Vec<String>,
}
