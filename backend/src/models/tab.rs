use serde::{Deserialize, Serialize};
use sqlx::types::time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct TabMetadata {
    pub work_title: String,
    pub composer: String,
    pub arranger: String,
    pub difficulty: i32,
    pub category: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PartInfo {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Note {
    pub pitch: i32,
    pub tpc: i32,
    pub fret: i32,
    pub string: i32,
    pub note_with_octave: String,
    pub duration: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Chord {
    pub notes: Vec<Note>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Measure {
    pub number: i32,
    pub chords: Vec<Chord>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Part {
    pub id: i32,
    pub name: String,
    pub measures: Vec<Measure>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScoreData {
    pub parts: Vec<Part>,
}

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

#[derive(Debug, Serialize)]
pub struct TabResponse {
    pub id: Uuid,
    pub metadata: TabMetadata,
    pub parts: Vec<PartInfo>,
    pub created_at: OffsetDateTime,
    pub last_used_at: OffsetDateTime,
    pub favorite_count: i32,
}


