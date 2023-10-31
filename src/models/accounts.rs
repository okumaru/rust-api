use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sqlx::FromRow;
use sqlx::types::BigDecimal;
use sqlx::types::chrono;

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct AccountModel {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
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
    pub balance: BigDecimal,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Default, Clone, FromRow, Deserialize, Serialize)]
pub struct NewAccount {
    pub name: String,
    pub description: Option<String>,
    pub balance: i32,
}


pub fn new_account(
    id: &i32,
    name: &String,
    description: &Option<String>,
    balance: &BigDecimal,
    created_at: &DateTime<Utc>,
    updated_at: &DateTime<Utc>,
) -> AccountModel {

    let new_balance = balance.clone().into_bigint_and_exponent();
    let int_balance = &new_balance.0.to_string().parse::<i64>().unwrap();

    AccountModel { 
        id: *id, 
        name: name.to_string(), 
        description: description.clone(), 
        balance: *int_balance,
        created_at: *created_at,
        updated_at: *updated_at,
    }
    
}