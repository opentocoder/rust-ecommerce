use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CartItem {
    pub user_id: Uuid,
    pub product_id: Uuid,
    pub quantity: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CartItemWithProduct {
    pub product_id: Uuid,
    pub product_name: String,
    pub product_price: f64,
    pub product_image_url: Option<String>,
    pub quantity: i32,
    pub subtotal: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Cart {
    pub user_id: Uuid,
    pub items: Vec<CartItemWithProduct>,
    pub total: f64,
}

impl Cart {
    pub fn calculate_total(&mut self) {
        self.total = self.items.iter().map(|item| item.subtotal).sum();
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn item_count(&self) -> usize {
        self.items.iter().map(|i| i.quantity as usize).sum()
    }
}
