use crate::db::Database;
use crate::error::AppError;

#[derive(Clone)]
pub struct AppState {
    pub db: Database,
}

impl AppState {
    pub async fn new() -> Result<Self, AppError> {
        let db = Database::new().await?;
        Ok(Self { db })
    }
}
