use anyhow::Result;
use sqlx::SqlitePool;
use uuid::Uuid;
use shared::{Cart, CartItem, CartItemWithProduct};

pub struct CartRepository;

impl CartRepository {
    pub async fn get_cart(pool: &SqlitePool, user_id: Uuid) -> Result<Cart> {
        let rows: Vec<(String, String, f64, Option<String>, i32)> = sqlx::query_as(
            r#"
            SELECT p.id, p.name, p.price, p.image_url, c.quantity
            FROM cart_items c
            JOIN products p ON c.product_id = p.id
            WHERE c.user_id = ? AND p.is_active = 1
            "#,
        )
        .bind(user_id.to_string())
        .fetch_all(pool)
        .await?;

        let items: Vec<CartItemWithProduct> = rows
            .into_iter()
            .map(|(product_id, name, price, image_url, quantity)| {
                CartItemWithProduct {
                    product_id: product_id.parse().unwrap_or_default(),
                    product_name: name,
                    product_price: price,
                    product_image_url: image_url,
                    quantity,
                    subtotal: price * quantity as f64,
                }
            })
            .collect();

        let total = items.iter().map(|i| i.subtotal).sum();

        Ok(Cart {
            user_id,
            items,
            total,
        })
    }

    pub async fn add_item(
        pool: &SqlitePool,
        user_id: Uuid,
        product_id: Uuid,
        quantity: i32,
    ) -> Result<()> {
        // Use INSERT OR REPLACE to handle duplicates
        sqlx::query(
            r#"
            INSERT INTO cart_items (user_id, product_id, quantity)
            VALUES (?, ?, ?)
            ON CONFLICT(user_id, product_id) DO UPDATE SET quantity = quantity + excluded.quantity
            "#,
        )
        .bind(user_id.to_string())
        .bind(product_id.to_string())
        .bind(quantity)
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn update_item_quantity(
        pool: &SqlitePool,
        user_id: Uuid,
        product_id: Uuid,
        quantity: i32,
    ) -> Result<bool> {
        let result = sqlx::query(
            "UPDATE cart_items SET quantity = ? WHERE user_id = ? AND product_id = ?",
        )
        .bind(quantity)
        .bind(user_id.to_string())
        .bind(product_id.to_string())
        .execute(pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn remove_item(
        pool: &SqlitePool,
        user_id: Uuid,
        product_id: Uuid,
    ) -> Result<bool> {
        let result = sqlx::query(
            "DELETE FROM cart_items WHERE user_id = ? AND product_id = ?",
        )
        .bind(user_id.to_string())
        .bind(product_id.to_string())
        .execute(pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn clear_cart(pool: &SqlitePool, user_id: Uuid) -> Result<()> {
        sqlx::query("DELETE FROM cart_items WHERE user_id = ?")
            .bind(user_id.to_string())
            .execute(pool)
            .await?;

        Ok(())
    }

    pub async fn get_cart_items(
        pool: &SqlitePool,
        user_id: Uuid,
    ) -> Result<Vec<CartItem>> {
        let rows: Vec<(String, String, i32)> = sqlx::query_as(
            "SELECT user_id, product_id, quantity FROM cart_items WHERE user_id = ?",
        )
        .bind(user_id.to_string())
        .fetch_all(pool)
        .await?;

        let items: Vec<CartItem> = rows
            .into_iter()
            .filter_map(|(user_id, product_id, quantity)| {
                Some(CartItem {
                    user_id: user_id.parse().ok()?,
                    product_id: product_id.parse().ok()?,
                    quantity,
                })
            })
            .collect();

        Ok(items)
    }
}
