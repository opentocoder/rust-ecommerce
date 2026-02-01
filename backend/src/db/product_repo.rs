use anyhow::Result;
use chrono::Utc;
use sqlx::SqlitePool;
use uuid::Uuid;
use shared::Product;

pub struct ProductRepository;

impl ProductRepository {
    pub async fn list(
        pool: &SqlitePool,
        page: u32,
        limit: u32,
        sort_by: Option<&str>,
        sort_order: Option<&str>,
    ) -> Result<(Vec<Product>, u32)> {
        let offset = (page - 1) * limit;

        let order_clause = match (sort_by, sort_order) {
            (Some("price"), Some("desc")) => "ORDER BY price DESC",
            (Some("price"), _) => "ORDER BY price ASC",
            (Some("name"), Some("desc")) => "ORDER BY name DESC",
            (Some("name"), _) => "ORDER BY name ASC",
            _ => "ORDER BY created_at DESC",
        };

        let query = format!(
            "SELECT id, name, description, price, stock, category, image_url, is_active, created_at, updated_at
             FROM products WHERE is_active = 1 {} LIMIT ? OFFSET ?",
            order_clause
        );

        let rows: Vec<(String, String, String, f64, i32, String, Option<String>, i32, String, String)> =
            sqlx::query_as(&query)
                .bind(limit as i64)
                .bind(offset as i64)
                .fetch_all(pool)
                .await?;

        let products: Vec<Product> = rows
            .into_iter()
            .filter_map(|row| Self::row_to_product(row).ok())
            .collect();

        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM products WHERE is_active = 1")
            .fetch_one(pool)
            .await?;

        Ok((products, count.0 as u32))
    }

    pub async fn get_by_id(pool: &SqlitePool, id: Uuid) -> Result<Option<Product>> {
        let row: Option<(String, String, String, f64, i32, String, Option<String>, i32, String, String)> =
            sqlx::query_as(
                "SELECT id, name, description, price, stock, category, image_url, is_active, created_at, updated_at
                 FROM products WHERE id = ?",
            )
            .bind(id.to_string())
            .fetch_optional(pool)
            .await?;

        match row {
            Some(row) => Ok(Some(Self::row_to_product(row)?)),
            None => Ok(None),
        }
    }

    pub async fn search(
        pool: &SqlitePool,
        query: &str,
        limit: u32,
    ) -> Result<Vec<Product>> {
        let search_pattern = format!("%{}%", query);

        let rows: Vec<(String, String, String, f64, i32, String, Option<String>, i32, String, String)> =
            sqlx::query_as(
                "SELECT id, name, description, price, stock, category, image_url, is_active, created_at, updated_at
                 FROM products WHERE is_active = 1 AND (name LIKE ? OR description LIKE ?) LIMIT ?",
            )
            .bind(&search_pattern)
            .bind(&search_pattern)
            .bind(limit as i64)
            .fetch_all(pool)
            .await?;

        let products: Vec<Product> = rows
            .into_iter()
            .filter_map(|row| Self::row_to_product(row).ok())
            .collect();

        Ok(products)
    }

    pub async fn filter_by_category(
        pool: &SqlitePool,
        category: &str,
        page: u32,
        limit: u32,
    ) -> Result<(Vec<Product>, u32)> {
        let offset = (page - 1) * limit;

        let rows: Vec<(String, String, String, f64, i32, String, Option<String>, i32, String, String)> =
            sqlx::query_as(
                "SELECT id, name, description, price, stock, category, image_url, is_active, created_at, updated_at
                 FROM products WHERE is_active = 1 AND category = ? ORDER BY created_at DESC LIMIT ? OFFSET ?",
            )
            .bind(category)
            .bind(limit as i64)
            .bind(offset as i64)
            .fetch_all(pool)
            .await?;

        let products: Vec<Product> = rows
            .into_iter()
            .filter_map(|row| Self::row_to_product(row).ok())
            .collect();

        let count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM products WHERE is_active = 1 AND category = ?",
        )
        .bind(category)
        .fetch_one(pool)
        .await?;

        Ok((products, count.0 as u32))
    }

    pub async fn list_categories(pool: &SqlitePool) -> Result<Vec<String>> {
        let rows: Vec<(String,)> = sqlx::query_as(
            "SELECT DISTINCT category FROM products WHERE is_active = 1 ORDER BY category",
        )
        .fetch_all(pool)
        .await?;

        Ok(rows.into_iter().map(|(c,)| c).collect())
    }

    pub async fn update_stock(pool: &SqlitePool, id: Uuid, quantity_change: i32) -> Result<()> {
        sqlx::query(
            "UPDATE products SET stock = stock + ?, updated_at = ? WHERE id = ?",
        )
        .bind(quantity_change)
        .bind(Utc::now().to_rfc3339())
        .bind(id.to_string())
        .execute(pool)
        .await?;

        Ok(())
    }

    fn row_to_product(
        row: (String, String, String, f64, i32, String, Option<String>, i32, String, String),
    ) -> Result<Product> {
        Ok(Product {
            id: row.0.parse()?,
            name: row.1,
            description: row.2,
            price: row.3,
            stock: row.4,
            category: row.5,
            image_url: row.6,
            is_active: row.7 == 1,
            created_at: chrono::DateTime::parse_from_rfc3339(&row.8)?.with_timezone(&Utc),
            updated_at: chrono::DateTime::parse_from_rfc3339(&row.9)?.with_timezone(&Utc),
        })
    }
}
