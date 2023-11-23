
use crate::models::bigdecimal_to_int;
use crate::models::accounts::{AccountModel, ExistAccount};
use crate::models::trx_cats::{TrxCatModel, ExistTrxCat};

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
    #[serde(rename = "balanceBefore")]
    pub balance_before: i64,
    #[serde(rename = "balanceAfter")]
    pub balance_after: i64,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
    pub accountid: i32,
    pub categoryid: i32,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct TrxModelWithAccCat {
    pub id: i32,
    pub credit: i64,
    pub debit: i64,
    pub description: Option<String>,
    #[serde(rename = "balanceBefore")]
    pub balance_before: i64,
    #[serde(rename = "balanceAfter")]
    pub balance_after: i64,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
    pub accountid: i32,
    pub categoryid: i32,
    pub account: AccountModel,
    pub category: TrxCatModel
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

#[derive(Debug, Default, Clone, FromRow)]
pub struct ExistTrxWithAccCat {
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
    pub account: ExistAccount,
    pub category: ExistTrxCat
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
}

pub fn build_model_from_exist(data: ExistTrxWithAccCat) -> TrxModelWithAccCat {
    let account = data.account;
    let category = data.category;

    return TrxModelWithAccCat {
        id: data.id,
        credit: bigdecimal_to_int(data.credit),
        debit: bigdecimal_to_int(data.debit),
        description: data.description,
        balance_before: bigdecimal_to_int(data.balance_before),
        balance_after: bigdecimal_to_int(data.balance_after),
        created_at: data.created_at,
        updated_at: data.updated_at,
        accountid: data.accountid,
        categoryid: data.categoryid,
        account: AccountModel {
            id: account.id,
            name: account.name,
            description: account.description,
            star: account.star,
            r#type: account.r#type,
            balance: bigdecimal_to_int(account.balance),
            created_at: account.created_at,
            updated_at: account.updated_at,
        },
        category: TrxCatModel {
            id: category.id,
            name: category.name,
            description: category.description,
            created_at: category.created_at,
            updated_at: category.updated_at,
            typeid: category.typeid,
            budget: None
        },
    }
}