use serde::{Deserialize, Serialize};
use sqlx::FromRow;
// use sqlx::types::chrono;
// use uuid::Uuid;

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct TrxsModel {
    pub id: u8,
    pub credit: u8,
    pub debit: u8,
    pub description: Option<String>,
    pub accountid: u8,
    pub categoryid: u8,
    // #[serde(rename = "createdAt")]
    // pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    // #[serde(rename = "updatedAt")]
    // pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}