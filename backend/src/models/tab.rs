use serde::{Deserialize, Serialize};
use sqlx::types::time::OffsetDateTime;
use uuid::Uuid;
use super::score::{Metadata, ScoreData, Part};

#[derive(Debug, Serialize, Deserialize)]
pub struct PartInfo {
    pub id: u32,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TabResponse {
    pub id: Uuid,
    pub metadata: Metadata,
    pub parts: Vec<PartInfo>,
    pub created_at: OffsetDateTime,
    pub last_used_at: OffsetDateTime,
    pub favorite_count: i32,
}

impl From<Part> for PartInfo {
    fn from(part: Part) -> Self {
        PartInfo {
            id: part.id,
            name: part.name,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Tab {
    pub id: Uuid,
    pub file_size: i64,
    pub metadata: Metadata,
    pub score_data: ScoreData,
    pub created_at: OffsetDateTime,
    pub last_used_at: OffsetDateTime,
    pub favorite_count: i32,
}
