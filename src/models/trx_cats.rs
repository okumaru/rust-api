use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct TrxCatsModel {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Default, Clone, FromRow)]
pub struct ExistTrxCat {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Default, Clone, FromRow, Deserialize, Serialize)]
pub struct NewTrxCat {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Default, Clone, FromRow, Deserialize, Serialize)]
pub struct UpdateTrxCat {
    pub name: Option<String>,
    pub description: Option<String>,
}