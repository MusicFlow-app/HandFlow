use std::env;
use crate::error::AppError;

#[derive(Clone, Debug)]
pub struct Config {
    pub database_url: String,
    pub port: u16,
    pub log_level: String,
    pub cors_origin: String,
    pub upload_limit: usize,  // in bytes
}

impl Config {
    pub fn new() -> Result<Self, AppError> {
        dotenv::dotenv().ok();

        let database_url = env::var("POSTGRESQL_ADDON_URI")
            .map_err(|_| AppError::Upload("Database URL not set".to_string()))?;

        let port = env::var("PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse()
            .map_err(|_| AppError::Upload("Invalid port number".to_string()))?;

        let log_level = env::var("RUST_LOG")
            .unwrap_or_else(|_| "info".to_string());

        let cors_origin = env::var("CORS_ORIGIN")
            .unwrap_or_else(|_| "http://localhost:5173".to_string());

        let upload_limit = env::var("UPLOAD_LIMIT")
            .unwrap_or_else(|_| "10485760".to_string())  // 10MB default
            .parse()
            .map_err(|_| AppError::Upload("Invalid upload limit".to_string()))?;

        Ok(Self {
            database_url,
            port,
            log_level,
            cors_origin,
            upload_limit,
        })
    }
}
