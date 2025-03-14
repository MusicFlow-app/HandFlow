use crate::db::Database;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<Database>,
}

impl AppState {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let db = Database::new().await?;
        Ok(Self {
            db: Arc::new(db),
        })
    }
}
