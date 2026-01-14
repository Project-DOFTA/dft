use axum::{
    routing::{get, post, put, delete},
    Router,
};
use sqlx::PgPool;
use tower_http::cors::{Any, CorsLayer};

use crate::handlers;

pub fn create_router(pool: PgPool) -> Router {
    // CORS configuration
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        // Health check
        .route("/health", get(health_check))
        
        // Auth routes (public)
        .route("/api/auth/register", post(handlers::auth::register))
        .route("/api/auth/login", post(handlers::auth::login))
        .route("/api/auth/profile", get(handlers::auth::get_profile))
        
        // Listing routes
        .route("/api/listings", get(handlers::listings::get_listings))
        .route("/api/listings", post(handlers::listings::create_listing))
        .route("/api/listings/:id", get(handlers::listings::get_listing))
        .route("/api/listings/:id", put(handlers::listings::update_listing))
        .route("/api/listings/:id", delete(handlers::listings::delete_listing))
        
        // Order routes
        .route("/api/orders", get(handlers::orders::get_my_orders))
        .route("/api/orders", post(handlers::orders::create_order))
        .route("/api/orders/:id", get(handlers::orders::get_order))
        .route("/api/orders/:id/status", put(handlers::orders::update_order_status))
        
        .layer(cors)
        .with_state(pool)
}

async fn health_check() -> &'static str {
    "OK"
}
