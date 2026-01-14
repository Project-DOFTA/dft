use std::env;

/// Application configuration
#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub server_host: String,
    pub server_port: u16,
    pub cooperative_fee_percentage: rust_decimal::Decimal,
}

impl Config {
    /// Load configuration from environment variables
    pub fn from_env() -> anyhow::Result<Self> {
        dotenvy::dotenv().ok();
        
        let database_url = env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://postgres:postgres@localhost/dofta".to_string());
        
        let jwt_secret = env::var("JWT_SECRET")
            .unwrap_or_else(|_| "your-secret-key-change-in-production".to_string());
        
        let server_host = env::var("SERVER_HOST")
            .unwrap_or_else(|_| "127.0.0.1".to_string());
        
        let server_port = env::var("SERVER_PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse()
            .unwrap_or(3000);
        
        let cooperative_fee_percentage = env::var("COOPERATIVE_FEE_PERCENTAGE")
            .unwrap_or_else(|_| "0.05".to_string())
            .parse()
            .unwrap_or_else(|_| rust_decimal::Decimal::new(5, 2)); // 0.05 = 5%
        
        Ok(Self {
            database_url,
            jwt_secret,
            server_host,
            server_port,
            cooperative_fee_percentage,
        })
    }
}
