
use crate::models::bigdecimal_to_int;

use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::FromRow;
use sqlx::types::BigDecimal;

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct AccountModel {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub star: bool,
    pub r#type: String,
    pub balance: i64,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Default, Clone, FromRow)]
pub struct ExistAccount {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub star: bool,
    pub r#type: String,
    pub balance: BigDecimal,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Default, Clone, FromRow, Deserialize, Serialize)]
pub struct NewAccount {
    pub name: String,
    pub description: Option<String>,
    pub star: bool,
    pub r#type: String,
    pub balance: i64,
}

#[derive(Debug, Default, Clone, FromRow, Deserialize, Serialize)]
pub struct UpdateAccount {
    pub name: Option<String>,
    pub description: Option<String>,
    pub star: Option<bool>,
    pub r#type: Option<String>,
    pub balance: Option<i64>,
}

pub fn new_account(
    id: &i32,
    name: &String,
    description: &Option<String>,
    star: &bool,
    r#type: &String,
    balance: &BigDecimal,
    created_at: &DateTime<Utc>,
    updated_at: &DateTime<Utc>,
) -> AccountModel {

    let int_balance = bigdecimal_to_int(balance.clone());

    AccountModel { 
        id: *id, 
        name: name.to_string(), 
        description: description.clone(), 
        star: star.clone(),
        r#type: r#type.clone(),
        balance: int_balance,
        created_at: *created_at,
        updated_at: *updated_at,
    }
    
}