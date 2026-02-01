mod auth;
mod db;
mod routes;
mod error;

use axum::{
    routing::{get, post, put, delete},
    Router,
};
use tower_http::cors::{CorsLayer, Any};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use std::sync::Arc;

pub struct AppState {
    pub db: db::Database,
    pub jwt_secret: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::EnvFilter::from_default_env()
            .add_directive("backend=debug".parse()?))
        .init();

    // Load environment variables
    dotenvy::dotenv().ok();

    // Initialize database
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite:./data.db?mode=rwc".to_string());
    let db = db::Database::new(&database_url).await?;

    // Run migrations
    db.migrate().await?;

    let jwt_secret = std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "your-secret-key-change-in-production".to_string());

    let state = Arc::new(AppState { db, jwt_secret });

    // CORS configuration
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Build routes
    let app = Router::new()
        // Health check
        .route("/health", get(|| async { "OK" }))
        // Auth routes
        .route("/api/auth/register", post(routes::auth::register))
        .route("/api/auth/login", post(routes::auth::login))
        // Product routes
        .route("/api/products", get(routes::products::list_products))
        .route("/api/products/:id", get(routes::products::get_product))
        .route("/api/products/search", get(routes::products::search_products))
        .route("/api/products/category/:category", get(routes::products::products_by_category))
        .route("/api/categories", get(routes::products::list_categories))
        // Cart routes (protected)
        .route("/api/cart", get(routes::cart::get_cart))
        .route("/api/cart", post(routes::cart::add_to_cart))
        .route("/api/cart/:product_id", put(routes::cart::update_cart_item))
        .route("/api/cart/:product_id", delete(routes::cart::remove_from_cart))
        // Order routes (protected)
        .route("/api/orders", get(routes::orders::list_orders))
        .route("/api/orders", post(routes::orders::create_order))
        .route("/api/orders/:id", get(routes::orders::get_order))
        .route("/api/orders/:id/cancel", put(routes::orders::cancel_order))
        // Middleware
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let addr = "0.0.0.0:3000";
    tracing::info!("Starting server on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
