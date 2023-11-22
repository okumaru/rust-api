use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::FromRow;
use sqlx::types::BigDecimal;

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct TrxCatBudgetModel {
    pub id: i32,
    pub periode: String,
    pub allocated: i64,
    pub spent: i64,
    pub available: i64,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
    pub categoryid: i32,
}

#[derive(PartialEq, Debug, Default, Clone, FromRow)]
pub struct ExistTrxCatBudget {
    pub id: i32,
    pub periode: String,
    pub allocated: BigDecimal,
    pub spent: BigDecimal,
    pub available: BigDecimal,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub categoryid: i32,
}

#[derive(PartialEq, Debug, Default, Clone, FromRow, Deserialize, Serialize)]
pub struct AddTrxCatBudget {
    pub periode: String,
    pub allocated: i64,
    pub spent: i64,
    pub available: i64
}

#[derive(Debug, Default, Clone, FromRow, Deserialize, Serialize)]
pub struct NewTrxCatBudget {
    pub periode: String,
    pub allocated: i64,
    pub spent: i64,
    pub available: i64,
    pub categoryid: i32,
}

#[derive(Debug, Default, Clone, FromRow, Deserialize, Serialize)]
pub struct UpdateTrxCatBudget {
    pub periode: Option<String>,
    pub allocated: Option<i64>,
    pub spent: Option<i64>, 
    pub available: Option<i64>,
    pub categoryid: Option<i32>,
}