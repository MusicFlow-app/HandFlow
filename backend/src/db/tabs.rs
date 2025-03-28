use sqlx::postgres::PgPool;
use sqlx::types::time::OffsetDateTime;
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};


use crate::models::tab::{TabMetadata, TabResponse, ScoreData, PartInfo};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Tab {
    pub id: Uuid,
    pub file_size: i64,
    pub metadata: serde_json::Value,
    pub score_data: serde_json::Value,
    pub created_at: OffsetDateTime,
    pub last_used_at: OffsetDateTime,
    pub favorite_count: i32,
}

impl Tab {
    pub fn to_response(&self) -> Result<TabResponse, serde_json::Error> {
        let metadata: TabMetadata = serde_json::from_value(self.metadata.clone())?;
        let score_data: ScoreData = serde_json::from_value(self.score_data.clone())?;
        
        let parts = score_data.parts.into_iter()
            .map(|part| PartInfo {
                id: part.id,
                name: part.name,
            })
            .collect();

        Ok(TabResponse {
            id: self.id,
            metadata,
            parts,
            created_at: self.created_at,
            last_used_at: self.last_used_at,
            favorite_count: self.favorite_count,
        })
    }
}

#[derive(Clone)]
pub struct Database {
    pool: PgPool,
}

impl Database {
    pub async fn new() -> Result<Self, sqlx::Error> {
        let config = crate::config::Config::new().map_err(|e| {
            sqlx::Error::Configuration(Box::new(e))
        })?;
        
        let pool = PgPool::connect(&config.database_url).await?;
        Ok(Self { pool })
    }

    pub async fn log_tab(
        &self,
        file_size: i64,
        metadata: serde_json::Value,
        score_data: serde_json::Value,
    ) -> Result<Tab, sqlx::Error> {
        // Generate deterministic UUID from metadata
        let mut hasher = Sha256::new();
        hasher.update(metadata.to_string().as_bytes());
        let hash = hasher.finalize();
        let mut uuid_bytes = [0u8; 16];
        uuid_bytes.copy_from_slice(&hash[..16]);
        uuid_bytes[6] = (uuid_bytes[6] & 0x0f) | 0x40;
        uuid_bytes[8] = (uuid_bytes[8] & 0x3f) | 0x80;
        let id = Uuid::from_bytes(uuid_bytes);

        // Try to insert new record, if it fails due to duplicate ID,
        // update the existing record's last_used_at timestamp
        let row = sqlx::query_as::<_, Tab>("
            INSERT INTO tabs (id, file_size, metadata, score_data, last_used_at, favorite_count)
            VALUES ($1, $2, $3, $4, CURRENT_TIMESTAMP, 0)
            ON CONFLICT (id) DO UPDATE
            SET last_used_at = CURRENT_TIMESTAMP,
                file_size = EXCLUDED.file_size,
                metadata = EXCLUDED.metadata,
                score_data = EXCLUDED.score_data
            RETURNING *
        ")
        .bind(id)
        .bind(file_size)
        .bind(metadata)
        .bind(score_data)
        .fetch_one(&self.pool)
        .await?;

        Ok(row)
    }

    #[allow(dead_code)]
    pub async fn get_tab(&self, id: Uuid) -> Result<Option<Tab>, sqlx::Error> {
        let row = sqlx::query_as::<_, Tab>("
            SELECT *
            FROM tabs
            WHERE id = $1
        ")
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row)
    }

    pub async fn update_favorite_count(&self, tab_id: Uuid, increment: bool) -> Result<(), sqlx::Error> {
        let delta = if increment { 1 } else { -1 };
        
        sqlx::query("
            UPDATE tabs 
            SET favorite_count = GREATEST(0, favorite_count + $1) 
            WHERE id = $2
        ")
        .bind(delta)
        .bind(tab_id)
        .execute(&self.pool)
        .await?
        ;
        
        Ok(())
    }

    pub async fn list_recent_tabs(
        &self,
        page: i64,
        per_page: i64,
        sort_by: &str,
        sort_order: &str
    ) -> Result<(Vec<Tab>, i64), sqlx::Error> {
        // Get total count
        let total_count: (i64,) = sqlx::query_as("
            SELECT COUNT(*) FROM tabs
        ")
        .fetch_one(&self.pool)
        .await?;

        // Build ORDER BY clause based on sort parameters
        let order_by = match sort_by {
            "title" => "metadata->>'title'",
            "favorite" => "favorite_count",
            _ => "created_at"
        };
        let order_direction = if sort_order == "asc" { "ASC" } else { "DESC" };
        
        let offset = (page - 1) * per_page;
        let query = format!(
            "SELECT * FROM tabs ORDER BY {} {} LIMIT $1 OFFSET $2",
            order_by,
            order_direction
        );
        
        let rows = sqlx::query_as::<_, Tab>(&query)
            .bind(per_page)
            .bind(offset)
            .fetch_all(&self.pool)
            .await?;

        Ok((rows, total_count.0))
    }
}
