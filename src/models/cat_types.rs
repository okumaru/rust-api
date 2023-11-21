use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct CatTypeModel {
    pub id: i32,
    pub r#type: String,
    pub description: Option<String>,
    pub icon: String,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Default, Clone, FromRow)]
pub struct ExistCatType {
    pub id: i32,
    pub r#type: String,
    pub description: Option<String>,
    pub icon: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Default, Clone, FromRow, Deserialize, Serialize)]
pub struct AddCatType {
    pub r#type: String,
    pub description: Option<String>,
    pub icon: String,
}

#[derive(Debug, Default, Clone, FromRow, Deserialize, Serialize)]
pub struct UpdateCatType {
    pub r#type: Option<String>,
    pub description: Option<String>,
    pub icon: Option<String>,
}