use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::types::BigDecimal;
use sqlx::FromRow;

#[derive(Debug, Default, Clone, FromRow, Deserialize, Serialize)]
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

#[derive(Debug, Default, Clone, FromRow, Deserialize, Serialize)]
pub struct CatTypeModelWithBudget {
    pub id: i32,
    pub r#type: String,
    pub description: Option<String>,
    pub icon: String,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
    pub allocated: i64,
    pub spent: i64,
    pub available: i64,
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

#[derive(Debug, Default, Clone, FromRow)]
pub struct ExistCatTypeWithBudget {
    pub id: i32,
    pub r#type: String,
    pub description: Option<String>,
    pub icon: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub allocated: Option<BigDecimal>,
    pub spent: Option<BigDecimal>,
    pub available: Option<BigDecimal>,
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