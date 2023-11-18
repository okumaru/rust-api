use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::FromRow;
use sqlx::types::BigDecimal;

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct TrxModel {
    pub id: i32,
    pub credit: i64,
    pub debit: i64,
    pub description: Option<String>,
    pub balance_before: i64,
    pub balance_after: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub accountid: i32,
    pub categoryid: i32,
}

#[derive(Debug, Default, Clone, FromRow)]
pub struct ExistTrx {
    pub id: i32,
    pub credit: BigDecimal,
    pub debit: BigDecimal,
    pub description: Option<String>,
    pub balance_before: BigDecimal,
    pub balance_after: BigDecimal,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub accountid: i32,
    pub categoryid: i32,
}

#[derive(Debug, Default, Clone, FromRow, Deserialize, Serialize)]
pub struct NewTrx {
    pub credit: i64,
    pub debit: i64,
    pub description: Option<String>,
    pub accountid: i32,
    pub categoryid: i32,
}

#[derive(Debug, Default, Clone, FromRow, Deserialize, Serialize)]
pub struct AddTrx {
    pub credit: i64,
    pub debit: i64,
    pub description: Option<String>,
    pub balance_before: i64,
    pub balance_after: i64,
    pub accountid: i32,
    pub categoryid: i32,
}

#[derive(Debug, Default, Clone, FromRow, Deserialize, Serialize)]
pub struct UpdateTrx {
    pub credit: Option<i64>,
    pub debit: Option<i64>,
    pub description: Option<String>,
    pub categoryid: Option<i32>,
}