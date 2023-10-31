use serde::{Deserialize, Serialize};
use sqlx::FromRow;
// use sqlx::types::chrono;
// use uuid::Uuid;

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct TrxCatsModel {
    pub id: u8,
    pub name: String,
    pub description: String,
    // #[serde(rename = "createdAt")]
    // pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    // #[serde(rename = "updatedAt")]
    // pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}