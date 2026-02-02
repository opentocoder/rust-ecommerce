mod auth;
mod db;
mod routes;
mod error;
mod rate_limit;

use axum::{
    routing::{get, post, put, delete},
    Router,
};
use tower_http::cors::{CorsLayer, AllowOrigin};
use tower_http::set_header::SetResponseHeaderLayer;
use axum::http::{header, HeaderValue, Method};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use std::sync::Arc;

pub struct AppState {
    pub db: db::Database,
    pub jwt_secret: String,
    pub login_rate_limiter: rate_limit::LoginRateLimiter,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging (default to info level in production)
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::EnvFilter::from_default_env()
            .add_directive("backend=info".parse()?))
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
        .expect("JWT_SECRET environment variable is required. Set a strong random secret (at least 32 characters).");

    // Validate JWT secret strength
    if jwt_secret.len() < 32 {
        panic!("JWT_SECRET must be at least 32 characters long for security");
    }

    let state = Arc::new(AppState {
        db,
        jwt_secret,
        login_rate_limiter: rate_limit::LoginRateLimiter::new(),
    });

    // CORS configuration - restricted to trusted origins
    let allowed_origins = std::env::var("ALLOWED_ORIGINS")
        .unwrap_or_else(|_| "http://localhost:8080,http://127.0.0.1:8080".to_string());

    let origins: Vec<_> = allowed_origins
        .split(',')
        .filter_map(|s| s.trim().parse().ok())
        .collect();

    let cors = CorsLayer::new()
        .allow_origin(AllowOrigin::list(origins))
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::OPTIONS])
        .allow_headers([header::AUTHORIZATION, header::CONTENT_TYPE, header::ACCEPT])
        .allow_credentials(true);

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
        // Security headers
        .layer(SetResponseHeaderLayer::overriding(
            header::X_CONTENT_TYPE_OPTIONS,
            HeaderValue::from_static("nosniff"),
        ))
        .layer(SetResponseHeaderLayer::overriding(
            header::X_FRAME_OPTIONS,
            HeaderValue::from_static("DENY"),
        ))
        .layer(SetResponseHeaderLayer::overriding(
            header::HeaderName::from_static("x-xss-protection"),
            HeaderValue::from_static("1; mode=block"),
        ))
        .layer(SetResponseHeaderLayer::overriding(
            header::HeaderName::from_static("referrer-policy"),
            HeaderValue::from_static("strict-origin-when-cross-origin"),
        ))
        .with_state(state);

    let addr = "0.0.0.0:3000";
    tracing::info!("Starting server on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<std::net::SocketAddr>(),
    )
    .await?;

    Ok(())
}
