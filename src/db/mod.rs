use sqlx::postgres::PgPool;
use sqlx::types::time::OffsetDateTime;
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};


#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct FileUsage {
    pub id: Uuid,
    pub filename: String,
    pub file_size: i64,
    pub metadata: serde_json::Value,
    pub mscx_content: String,
    pub created_at: OffsetDateTime,
    pub last_used_at: OffsetDateTime,
}

pub struct Database {
    pool: PgPool,
}

impl Database {
    pub async fn new() -> Result<Self, sqlx::Error> {
        dotenv::dotenv().ok();
        let database_url = std::env::var("POSTGRESQL_ADDON_URI")
            .expect("POSTGRESQL_ADDON_URI must be set");
        
        let pool = PgPool::connect(&database_url).await?;
        
        // Run migrations
        sqlx::migrate!("./migrations")
            .run(&pool)
            .await?;
        
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

    pub async fn log_file_usage(
        &self,
        filename: &str,
        file_size: i64,
        metadata: serde_json::Value,
        mscx_content: &str,
    ) -> Result<FileUsage, sqlx::Error> {
        // Generate deterministic UUID from content
        let id = Self::generate_deterministic_uuid(mscx_content);

        // Try to insert new record, if it fails due to duplicate ID,
        // update the existing record's last_used_at timestamp
        let row = sqlx::query_as::<_, FileUsage>("
            INSERT INTO file_usage (id, filename, file_size, metadata, mscx_content, last_used_at)
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
    pub async fn get_file_usage(&self, id: Uuid) -> Result<Option<FileUsage>, sqlx::Error> {
        let row = sqlx::query_as::<_, FileUsage>("
            SELECT *
            FROM file_usage
            WHERE id = $1
        ")
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row)
    }

    pub async fn list_recent_usage(&self, limit: i64) -> Result<Vec<FileUsage>, sqlx::Error> {
        let rows = sqlx::query_as::<_, FileUsage>("
            SELECT *
            FROM file_usage
            ORDER BY last_used_at DESC
            LIMIT $1
        ")
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows)
    }
}
