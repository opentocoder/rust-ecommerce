//! Seed database with sample data

use chrono::Utc;
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use uuid::Uuid;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite:./data.db?mode=rwc".to_string());

    println!("Connecting to database: {}", database_url);

    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect(&database_url)
        .await?;

    // Run migrations first
    create_tables(&pool).await?;

    // Seed products
    seed_products(&pool).await?;

    // Seed admin user
    seed_admin(&pool).await?;

    println!("Database seeded successfully!");
    Ok(())
}

async fn create_tables(pool: &SqlitePool) -> anyhow::Result<()> {
    println!("Creating tables...");

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id TEXT PRIMARY KEY,
            username TEXT NOT NULL UNIQUE,
            email TEXT NOT NULL UNIQUE,
            password_hash TEXT NOT NULL,
            role TEXT NOT NULL DEFAULT 'user',
            created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS products (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            description TEXT NOT NULL,
            price REAL NOT NULL,
            stock INTEGER NOT NULL DEFAULT 0,
            category TEXT NOT NULL,
            image_url TEXT,
            is_active INTEGER NOT NULL DEFAULT 1,
            created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS cart_items (
            user_id TEXT NOT NULL,
            product_id TEXT NOT NULL,
            quantity INTEGER NOT NULL DEFAULT 1,
            PRIMARY KEY (user_id, product_id)
        )
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS orders (
            id TEXT PRIMARY KEY,
            user_id TEXT NOT NULL,
            status TEXT NOT NULL DEFAULT 'pending',
            total REAL NOT NULL,
            created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS order_items (
            id TEXT PRIMARY KEY,
            order_id TEXT NOT NULL,
            product_id TEXT NOT NULL,
            product_name TEXT NOT NULL,
            quantity INTEGER NOT NULL,
            price REAL NOT NULL,
            subtotal REAL NOT NULL
        )
        "#,
    )
    .execute(pool)
    .await?;

    Ok(())
}

async fn seed_products(pool: &SqlitePool) -> anyhow::Result<()> {
    println!("Seeding products...");

    let products = vec![
        ("Wireless Bluetooth Headphones", "High-quality wireless headphones with noise cancellation and 30-hour battery life.", 79.99, 50, "Electronics", "https://images.unsplash.com/photo-1505740420928-5e560c06d30e?w=400"),
        ("Mechanical Gaming Keyboard", "RGB backlit mechanical keyboard with Cherry MX switches for ultimate gaming experience.", 149.99, 30, "Electronics", "https://images.unsplash.com/photo-1511467687858-23d96c32e4ae?w=400"),
        ("Ergonomic Office Chair", "Premium ergonomic chair with lumbar support and adjustable armrests.", 299.99, 20, "Furniture", "https://images.unsplash.com/photo-1580480055273-228ff5388ef8?w=400"),
        ("Standing Desk Converter", "Adjustable standing desk converter for healthier work habits.", 199.99, 15, "Furniture", "https://images.unsplash.com/photo-1518455027359-f3f8164ba6bd?w=400"),
        ("Wireless Mouse", "Precision wireless mouse with customizable DPI settings.", 49.99, 100, "Electronics", "https://images.unsplash.com/photo-1527864550417-7fd91fc51a46?w=400"),
        ("USB-C Hub", "7-in-1 USB-C hub with HDMI, USB 3.0, and SD card reader.", 39.99, 75, "Electronics", "https://images.unsplash.com/photo-1625723044792-2d889f7ac2f9?w=400"),
        ("Laptop Stand", "Aluminum laptop stand with adjustable height and angle.", 59.99, 40, "Accessories", "https://images.unsplash.com/photo-1527443224154-c4a3942d3acf?w=400"),
        ("Desk Lamp", "LED desk lamp with adjustable brightness and color temperature.", 34.99, 60, "Accessories", "https://images.unsplash.com/photo-1507473885765-e6ed057f782c?w=400"),
        ("Webcam HD", "1080p HD webcam with built-in microphone for video conferencing.", 69.99, 45, "Electronics", "https://images.unsplash.com/photo-1587826080692-f439cd0b70da?w=400"),
        ("Monitor Arm", "Dual monitor arm with full motion and cable management.", 89.99, 25, "Accessories", "https://images.unsplash.com/photo-1593640408182-31c70c8268f5?w=400"),
        ("Notebook Set", "Premium leather-bound notebook set with pen holder.", 24.99, 80, "Office Supplies", "https://images.unsplash.com/photo-1531346878377-a5be20888e57?w=400"),
        ("Wireless Charger", "Fast wireless charging pad compatible with all Qi devices.", 29.99, 90, "Electronics", "https://images.unsplash.com/photo-1586816879360-004f5b0c51e5?w=400"),
    ];

    let now = Utc::now().to_rfc3339();

    for (name, description, price, stock, category, image_url) in products {
        let id = Uuid::new_v4();

        sqlx::query(
            r#"
            INSERT OR IGNORE INTO products (id, name, description, price, stock, category, image_url, is_active, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, 1, ?, ?)
            "#,
        )
        .bind(id.to_string())
        .bind(name)
        .bind(description)
        .bind(price)
        .bind(stock)
        .bind(category)
        .bind(image_url)
        .bind(&now)
        .bind(&now)
        .execute(pool)
        .await?;

        println!("  Added: {}", name);
    }

    Ok(())
}

async fn seed_admin(pool: &SqlitePool) -> anyhow::Result<()> {
    println!("Seeding admin user...");

    // Check if admin exists
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users WHERE email = 'admin@example.com'")
        .fetch_one(pool)
        .await?;

    if count.0 > 0 {
        println!("  Admin user already exists");
        return Ok(());
    }

    let id = Uuid::new_v4();
    let now = Utc::now().to_rfc3339();

    // Password: admin123 (hashed with argon2)
    // In production, use proper password hashing
    let password_hash = "$argon2id$v=19$m=19456,t=2,p=1$YWRtaW5zYWx0MTIzNDU2$bH8DnF8NKgYQgZpG8xyG8Q";

    sqlx::query(
        r#"
        INSERT INTO users (id, username, email, password_hash, role, created_at)
        VALUES (?, 'admin', 'admin@example.com', ?, 'admin', ?)
        "#,
    )
    .bind(id.to_string())
    .bind(password_hash)
    .bind(&now)
    .execute(pool)
    .await?;

    println!("  Admin user created: admin@example.com");
    println!("  Note: For testing, register a new user through the app");

    Ok(())
}
