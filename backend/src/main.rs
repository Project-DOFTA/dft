use dofta::{config::Config, db::Database, routes};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "dofta=debug,tower_http=debug,axum=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    
    tracing::info!("🌾 Starting DOFTA Farmers Cooperative Platform");
    
    // Load configuration
    let config = Config::from_env()?;
    tracing::info!("✅ Configuration loaded");
    
    // Initialize database connection pool
    let db = Database::new(&config.database_url).await?;
    tracing::info!("✅ Database connection pool established");
    
    // Run migrations
    db.migrate().await?;
    tracing::info!("✅ Database migrations completed");
    
    // Health check
    db.health_check().await?;
    tracing::info!("✅ Database health check passed");
    
    // Create router
    let app = routes::create_router(db.pool().clone());
    
    // Start server
    let addr = format!("{}:{}", config.server_host, config.server_port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    
    tracing::info!("🚀 DOFTA API server running on http://{}", addr);
    tracing::info!("📡 Health check: http://{}/health", addr);
    tracing::info!("🔐 API endpoints: http://{}/api/*", addr);
    
    axum::serve(listener, app).await?;
    
    Ok(())
}
