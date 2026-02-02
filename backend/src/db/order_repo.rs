use anyhow::Result;
use chrono::Utc;
use sqlx::SqlitePool;
use uuid::Uuid;
use shared::{Order, OrderItem, OrderStatus, OrderWithItems, CartItemWithProduct};

pub struct OrderRepository;

impl OrderRepository {
    /// Atomic order creation with stock validation and update
    /// Uses database transaction to prevent race conditions
    pub async fn create_order_atomic(
        pool: &SqlitePool,
        user_id: Uuid,
        cart_items: &[CartItemWithProduct],
    ) -> Result<OrderWithItems> {
        let mut tx = pool.begin().await?;

        let order_id = Uuid::new_v4();
        let now = Utc::now();
        let mut order_items = Vec::new();
        let mut total: f64 = 0.0;

        // Verify stock and collect order items within transaction
        for item in cart_items {
            // Lock the row by selecting FOR UPDATE (SQLite handles this implicitly in transaction)
            let row: Option<(i32, String)> = sqlx::query_as(
                "SELECT stock, name FROM products WHERE id = ? AND is_active = 1"
            )
            .bind(item.product_id.to_string())
            .fetch_optional(&mut *tx)
            .await?;

            let (stock, product_name) = row.ok_or_else(|| {
                anyhow::anyhow!("Product {} not found or inactive", item.product_id)
            })?;

            if stock < item.quantity {
                return Err(anyhow::anyhow!("Insufficient stock for {}: requested {}, available {}",
                    product_name, item.quantity, stock));
            }

            // Update stock immediately within transaction
            sqlx::query(
                "UPDATE products SET stock = stock - ?, updated_at = ? WHERE id = ?"
            )
            .bind(item.quantity)
            .bind(now.to_rfc3339())
            .bind(item.product_id.to_string())
            .execute(&mut *tx)
            .await?;

            let subtotal = item.quantity as f64 * item.product_price;
            total += subtotal;

            order_items.push((item.product_id, product_name, item.quantity, item.product_price, subtotal));
        }

        // Create order
        sqlx::query(
            r#"
            INSERT INTO orders (id, user_id, status, total, created_at, updated_at)
            VALUES (?, ?, 'pending', ?, ?, ?)
            "#,
        )
        .bind(order_id.to_string())
        .bind(user_id.to_string())
        .bind(total)
        .bind(now.to_rfc3339())
        .bind(now.to_rfc3339())
        .execute(&mut *tx)
        .await?;

        // Create order items
        let mut result_items = Vec::new();
        for (product_id, product_name, quantity, price, subtotal) in order_items {
            let item_id = Uuid::new_v4();

            sqlx::query(
                r#"
                INSERT INTO order_items (id, order_id, product_id, product_name, quantity, price, subtotal)
                VALUES (?, ?, ?, ?, ?, ?, ?)
                "#,
            )
            .bind(item_id.to_string())
            .bind(order_id.to_string())
            .bind(product_id.to_string())
            .bind(&product_name)
            .bind(quantity)
            .bind(price)
            .bind(subtotal)
            .execute(&mut *tx)
            .await?;

            result_items.push(OrderItem {
                id: item_id,
                order_id,
                product_id,
                product_name,
                quantity,
                price,
                subtotal,
            });
        }

        // Clear cart within transaction
        sqlx::query("DELETE FROM cart_items WHERE user_id = ?")
            .bind(user_id.to_string())
            .execute(&mut *tx)
            .await?;

        // Commit transaction
        tx.commit().await?;

        let order = Order {
            id: order_id,
            user_id,
            status: OrderStatus::Pending,
            total,
            created_at: now,
            updated_at: now,
        };

        Ok(OrderWithItems {
            order,
            items: result_items,
        })
    }

    pub async fn create(
        pool: &SqlitePool,
        user_id: Uuid,
        items: Vec<(Uuid, String, i32, f64)>, // (product_id, name, quantity, price)
    ) -> Result<OrderWithItems> {
        let order_id = Uuid::new_v4();
        let now = Utc::now();
        let total: f64 = items.iter().map(|(_, _, qty, price)| *qty as f64 * price).sum();

        // Create order
        sqlx::query(
            r#"
            INSERT INTO orders (id, user_id, status, total, created_at, updated_at)
            VALUES (?, ?, 'pending', ?, ?, ?)
            "#,
        )
        .bind(order_id.to_string())
        .bind(user_id.to_string())
        .bind(total)
        .bind(now.to_rfc3339())
        .bind(now.to_rfc3339())
        .execute(pool)
        .await?;

        // Create order items
        let mut order_items = Vec::new();
        for (product_id, product_name, quantity, price) in items {
            let item_id = Uuid::new_v4();
            let subtotal = quantity as f64 * price;

            sqlx::query(
                r#"
                INSERT INTO order_items (id, order_id, product_id, product_name, quantity, price, subtotal)
                VALUES (?, ?, ?, ?, ?, ?, ?)
                "#,
            )
            .bind(item_id.to_string())
            .bind(order_id.to_string())
            .bind(product_id.to_string())
            .bind(&product_name)
            .bind(quantity)
            .bind(price)
            .bind(subtotal)
            .execute(pool)
            .await?;

            order_items.push(OrderItem {
                id: item_id,
                order_id,
                product_id,
                product_name,
                quantity,
                price,
                subtotal,
            });
        }

        let order = Order {
            id: order_id,
            user_id,
            status: OrderStatus::Pending,
            total,
            created_at: now,
            updated_at: now,
        };

        Ok(OrderWithItems {
            order,
            items: order_items,
        })
    }

    pub async fn list_by_user(
        pool: &SqlitePool,
        user_id: Uuid,
    ) -> Result<Vec<Order>> {
        let rows: Vec<(String, String, String, f64, String, String)> = sqlx::query_as(
            r#"
            SELECT id, user_id, status, total, created_at, updated_at
            FROM orders WHERE user_id = ? ORDER BY created_at DESC
            "#,
        )
        .bind(user_id.to_string())
        .fetch_all(pool)
        .await?;

        let orders: Vec<Order> = rows
            .into_iter()
            .filter_map(|row| Self::row_to_order(row).ok())
            .collect();

        Ok(orders)
    }

    pub async fn get_by_id(pool: &SqlitePool, id: Uuid) -> Result<Option<OrderWithItems>> {
        let row: Option<(String, String, String, f64, String, String)> = sqlx::query_as(
            r#"
            SELECT id, user_id, status, total, created_at, updated_at
            FROM orders WHERE id = ?
            "#,
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await?;

        match row {
            Some(row) => {
                let order = Self::row_to_order(row)?;
                let items = Self::get_order_items(pool, id).await?;
                Ok(Some(OrderWithItems { order, items }))
            }
            None => Ok(None),
        }
    }

    pub async fn get_order_items(pool: &SqlitePool, order_id: Uuid) -> Result<Vec<OrderItem>> {
        let rows: Vec<(String, String, String, String, i32, f64, f64)> = sqlx::query_as(
            r#"
            SELECT id, order_id, product_id, product_name, quantity, price, subtotal
            FROM order_items WHERE order_id = ?
            "#,
        )
        .bind(order_id.to_string())
        .fetch_all(pool)
        .await?;

        let items: Vec<OrderItem> = rows
            .into_iter()
            .filter_map(|(id, order_id, product_id, product_name, quantity, price, subtotal)| {
                Some(OrderItem {
                    id: id.parse().ok()?,
                    order_id: order_id.parse().ok()?,
                    product_id: product_id.parse().ok()?,
                    product_name,
                    quantity,
                    price,
                    subtotal,
                })
            })
            .collect();

        Ok(items)
    }

    pub async fn update_status(
        pool: &SqlitePool,
        id: Uuid,
        status: OrderStatus,
    ) -> Result<bool> {
        let status_str = match status {
            OrderStatus::Pending => "pending",
            OrderStatus::Paid => "paid",
            OrderStatus::Shipped => "shipped",
            OrderStatus::Delivered => "delivered",
            OrderStatus::Cancelled => "cancelled",
        };

        let result = sqlx::query(
            "UPDATE orders SET status = ?, updated_at = ? WHERE id = ?",
        )
        .bind(status_str)
        .bind(Utc::now().to_rfc3339())
        .bind(id.to_string())
        .execute(pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    fn row_to_order(
        row: (String, String, String, f64, String, String),
    ) -> Result<Order> {
        let status = match row.2.as_str() {
            "pending" => OrderStatus::Pending,
            "paid" => OrderStatus::Paid,
            "shipped" => OrderStatus::Shipped,
            "delivered" => OrderStatus::Delivered,
            "cancelled" => OrderStatus::Cancelled,
            _ => OrderStatus::Pending,
        };

        Ok(Order {
            id: row.0.parse()?,
            user_id: row.1.parse()?,
            status,
            total: row.3,
            created_at: chrono::DateTime::parse_from_rfc3339(&row.4)?.with_timezone(&Utc),
            updated_at: chrono::DateTime::parse_from_rfc3339(&row.5)?.with_timezone(&Utc),
        })
    }
}
