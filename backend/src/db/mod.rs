use sqlx::postgres::PgPool;
use sqlx::types::time::OffsetDateTime;
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};


#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Tab {
    pub id: Uuid,
    pub filename: String,
    pub file_size: i64,
    pub metadata: serde_json::Value,
    pub mscx_content: String,
    pub created_at: OffsetDateTime,
    pub last_used_at: OffsetDateTime,
}

#[derive(Clone)]
pub struct Database {
    pool: PgPool,
}

impl Database {
    pub async fn new() -> Result<Self, sqlx::Error> {
        dotenv::dotenv().ok();
        let database_url = std::env::var("POSTGRESQL_ADDON_URI")
            .expect("POSTGRESQL_ADDON_URI must be set");
        
        let pool = PgPool::connect(&database_url).await?;
        Ok(Self { pool })
    }

    fn generate_deterministic_uuid(content: &str) -> Uuid {
        let mut hasher = Sha256::new();
        hasher.update(content.as_bytes());
        let hash = hasher.finalize();
        
        // Use first 16 bytes of hash to create UUID (UUID is 128 bits = 16 bytes)
        let mut uuid_bytes = [0u8; 16];
        uuid_bytes.copy_from_slice(&hash[..16]);
        
        // Set version (4) and variant bits according to RFC 4122
        uuid_bytes[6] = (uuid_bytes[6] & 0x0f) | 0x40; // Version 4
        uuid_bytes[8] = (uuid_bytes[8] & 0x3f) | 0x80; // Variant 1
        
        Uuid::from_bytes(uuid_bytes)
    }

    pub async fn log_tab(
        &self,
        filename: &str,
        file_size: i64,
        metadata: serde_json::Value,
        mscx_content: &str,
    ) -> Result<Tab, sqlx::Error> {
        // Generate deterministic UUID from content
        let id = Self::generate_deterministic_uuid(mscx_content);

        // Try to insert new record, if it fails due to duplicate ID,
        // update the existing record's last_used_at timestamp
        let row = sqlx::query_as::<_, Tab>("
            INSERT INTO tabs (id, filename, file_size, metadata, mscx_content, last_used_at)
            VALUES ($1, $2, $3, $4, $5, CURRENT_TIMESTAMP)
            ON CONFLICT (id) DO UPDATE
            SET last_used_at = CURRENT_TIMESTAMP,
                filename = EXCLUDED.filename,
                file_size = EXCLUDED.file_size,
                metadata = EXCLUDED.metadata
            RETURNING *
        ")
        .bind(id)
        .bind(filename)
        .bind(file_size)
        .bind(metadata)
        .bind(mscx_content)
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
