
use crate::models::bigdecimal_to_int;
use crate::models::cat_types::{CatTypeModel, ExistCatType};
use crate::models::trx_cat_budgets::{TrxCatBudgetModel, ExistTrxCatBudget, AddTrxCatBudget};

use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct TrxCatModel {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
    pub typeid: i32,
    pub budget: Option<TrxCatBudgetModel>
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct TrxCatModelWithType {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
    pub typeid: i32,
    pub r#type: CatTypeModel,
    pub budget: Option<TrxCatBudgetModel>
}

#[derive(Debug, Default, Clone, FromRow)]
pub struct ExistTrxCat {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub typeid: i32
}

#[derive(Debug, Default, Clone, FromRow)]
pub struct ExistTrxCatWithBudget {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub typeid: i32,
    pub budget: Option<ExistTrxCatBudget>
}

#[derive(Debug, Default, Clone, FromRow)]
pub struct ExistTrxCatWithBudgetType {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub typeid: i32,
    pub r#type: ExistCatType,
    pub budget: Option<ExistTrxCatBudget>
}

#[derive(Debug, Default, Clone, FromRow, Deserialize, Serialize)]
pub struct AddTrxCat {
    pub name: String,
    pub description: Option<String>,
    pub typeid: i32,
    pub budget: Option<AddTrxCatBudget>
}

#[derive(Debug, Default, Clone, FromRow, Deserialize, Serialize)]
pub struct UpdateTrxCat {
    pub name: Option<String>,
    pub description: Option<String>,
}

pub fn detail_model_from_exist(data: ExistTrxCatWithBudgetType) -> TrxCatModelWithType {
    let mut budget: Option<TrxCatBudgetModel> = None;
    let data_type: ExistCatType = data.r#type;

    if data.budget.clone() != None {
        let budget_field = data.budget.clone().unwrap();
        budget = Some( TrxCatBudgetModel {
            id: budget_field.id,
            periode: budget_field.periode,
            allocated: bigdecimal_to_int(budget_field.allocated),
            spent: bigdecimal_to_int(budget_field.spent),
            available: bigdecimal_to_int(budget_field.available),
            created_at: budget_field.created_at,
            updated_at: budget_field.updated_at,
            categoryid: budget_field.categoryid,
        } )
    }

    return TrxCatModelWithType {
        id: data.id, 
        name: data.name, 
        description: data.description, 
        created_at: data.created_at,
        updated_at: data.updated_at,
        typeid: data.typeid,
        r#type: CatTypeModel{
            id: data_type.id,
            r#type: data_type.r#type,
            description: data_type.description,
            icon: data_type.icon,
            created_at: data_type.created_at,
            updated_at: data_type.updated_at,
        },
        budget: budget
    }
}

pub fn build_model_from_exist(data: ExistTrxCatWithBudget) -> TrxCatModel {
    let mut budget: Option<TrxCatBudgetModel> = None;

    if data.budget.clone() != None {
        let budget_field = data.budget.clone().unwrap();
        budget = Some( TrxCatBudgetModel {
            id: budget_field.id,
            periode: budget_field.periode,
            allocated: bigdecimal_to_int(budget_field.allocated),
            spent: bigdecimal_to_int(budget_field.spent),
            available: bigdecimal_to_int(budget_field.available),
            created_at: budget_field.created_at,
            updated_at: budget_field.updated_at,
            categoryid: budget_field.categoryid,
        } )
    }

    return TrxCatModel {
        id: data.id, 
        name: data.name, 
        description: data.description, 
        created_at: data.created_at,
        updated_at: data.updated_at,
        typeid: data.typeid,
        budget: budget
    }
}

pub fn build_exist_trx_cat_budget_type(
    trx_cat: ExistTrxCat,
    cat_type: ExistCatType,
    cat_budget: Option<ExistTrxCatBudget>,
) -> ExistTrxCatWithBudgetType {

    ExistTrxCatWithBudgetType {
        id: trx_cat.id.clone(),
        name: trx_cat.name.clone(),
        description: trx_cat.description.clone(),
        created_at: trx_cat.created_at.clone(),
        updated_at: trx_cat.updated_at.clone(),
        typeid: trx_cat.typeid.clone(),
        r#type: cat_type,
        budget: cat_budget
    }
    
}

pub fn build_exist_trx_cat_budget(
    trx_cat: ExistTrxCat,
    cat_budget: Option<ExistTrxCatBudget>
) -> ExistTrxCatWithBudget {

    ExistTrxCatWithBudget {
        id: trx_cat.id.clone(),
        name: trx_cat.name.clone(),
        description: trx_cat.description.clone(),
        created_at: trx_cat.created_at.clone(),
        updated_at: trx_cat.updated_at.clone(),
        typeid: trx_cat.typeid.clone(),
        budget: cat_budget
    }
    
}