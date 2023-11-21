
use crate::models::trx_cats::{ ExistTrxCat, ExistTrxCatWithBudget, build_exist_trx_cat_budget };
use crate::models::trx_cat_budgets::{ ExistTrxCatBudget, AddTrxCatBudget, NewTrxCatBudget, UpdateTrxCatBudget };
use crate::repositories::{ Executor, UpdateQuery };

use futures_util::{future::BoxFuture, FutureExt};
use sqlx::{MySql, MySqlPool};
use sqlx_mysql::MySqlQueryResult;

#[async_trait::async_trait]
pub trait Trait: Send + Sync + TrxCatBudgetTrait {
    fn clone_boxed(&self) -> Box<dyn Trait>;
    async fn start_transaction(
        &self,
    ) -> Result<Box<dyn TransactionTrait>, Box<dyn std::error::Error + Send + Sync + 'static>>;
}

#[async_trait::async_trait]
pub trait TransactionTrait: Send + Sync + TrxCatBudgetTrait {
    async fn commit(
        self: Box<Self>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>>;
}

#[async_trait::async_trait]
pub trait TrxCatBudgetTrait {
    async fn trx_cat_budget_list(
        &mut self,
    ) -> Result<Vec<ExistTrxCatBudget>, Box<dyn std::error::Error + Send + Sync + 'static>>;
    async fn trx_cat_budget_detail(
        &mut self,
        id: i32,
    ) -> Result<ExistTrxCatBudget, Box<dyn std::error::Error + Send + Sync + 'static>>;
    async fn trx_cat_budget_add(
        &mut self,
        budget: NewTrxCatBudget,
    ) -> Result<ExistTrxCatBudget, Box<dyn std::error::Error + Send + Sync + 'static>>;
    async fn trx_cat_budget_update(
        &mut self,
        id: i32,
        budget: UpdateTrxCatBudget,
    ) -> Result<ExistTrxCatBudget, Box<dyn std::error::Error + Send + Sync + 'static>>;
    async fn trx_cat_budget_delete(
        &mut self,
        id: i32,
    ) -> Result<ExistTrxCatBudget, Box<dyn std::error::Error + Send + Sync + 'static>>;
}

#[derive(Debug, Clone)]
pub struct TrxCatBudgetRepo<E = MySqlPool> {
    db: E,
}

impl TrxCatBudgetRepo {
    pub fn new(pool: MySqlPool) -> Self {
        Self { db: pool }
    }
}

#[async_trait::async_trait]
impl Trait for TrxCatBudgetRepo {
    fn clone_boxed(&self) -> Box<dyn Trait> {
        Box::new(Clone::clone(self))
    }

    async fn start_transaction(
        &self,
    ) -> Result<Box<dyn TransactionTrait>, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let tx = self.db.begin().await?;

        Ok(Box::new(TrxCatBudgetRepo { db: tx }))
    }
}

#[async_trait::async_trait]
impl TransactionTrait for TrxCatBudgetRepo<sqlx::Transaction<'static, MySql>> {
    async fn commit(
        self: Box<Self>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.db.commit().await?;

        Ok(())
    }
}

#[async_trait::async_trait]
impl<E: 'static + Executor> TrxCatBudgetTrait for TrxCatBudgetRepo<E> {
    async fn trx_cat_budget_list(
        &mut self,
    ) -> Result<Vec<ExistTrxCatBudget>, Box<dyn std::error::Error + Send + Sync + 'static>> {

        let budget = query_list_trx_cat_budget(&mut self.db).await;
        Ok(budget)
    }

    async fn trx_cat_budget_detail(
        &mut self,
        id: i32,
    ) -> Result<ExistTrxCatBudget, Box<dyn std::error::Error + Send + Sync + 'static>> {

        let budget = query_detail_trx_cat_budget(&mut self.db, id).await;
        Ok(budget)
    }

    async fn trx_cat_budget_add(
        &mut self,
        add_budget: NewTrxCatBudget,
    ) -> Result<ExistTrxCatBudget, Box<dyn std::error::Error + Send + Sync + 'static>> {

        // add trx cat budget
        let add = query_add_trx_cat_budget(&mut self.db, add_budget).await;
        let budget_id = i32::try_from(add.last_insert_id()).unwrap();

        // detail trx cat budget
        let budget = query_detail_trx_cat_budget(&mut self.db, budget_id).await;
        Ok(budget)
    }

    async fn trx_cat_budget_update(
        &mut self,
        id: i32,
        budget: UpdateTrxCatBudget,
    ) -> Result<ExistTrxCatBudget, Box<dyn std::error::Error + Send + Sync + 'static>> {

        // update trx cat budget
        let _ = query_update_trx_cat_budget(&mut self.db, id, budget).await;

        // trx cat budget detail
        let budget = query_detail_trx_cat_budget(&mut self.db, id).await;
        Ok(budget)
    }

    async fn trx_cat_budget_delete(
        &mut self,
        id: i32,
    ) -> Result<ExistTrxCatBudget, Box<dyn std::error::Error + Send + Sync + 'static>> {

        // trx cat budget detail
        let budget = query_detail_trx_cat_budget(&mut self.db, id).await;

        // delete trx cat budget
        let _ = query_delete_cat_budget(&mut self.db, id).await;

        Ok(budget)
    }
}

pub fn query_list_trx_cat_budget<'a>(
    db: &'a mut impl Executor,
) -> BoxFuture<'a, Vec<ExistTrxCatBudget>> {
    async move {
        let mut query = sqlx::QueryBuilder::new(r#"SELECT * FROM tblcategorybudgets"#);

        let trxs = query
            .build_query_as::<ExistTrxCatBudget>()
            .fetch_all(db.as_executor())
            .await
            .unwrap();

        trxs
    }
    .boxed()
}

pub fn query_detail_trx_cat_budget<'a>(
    db: &'a mut impl Executor,
    id: i32
) -> BoxFuture<'a, ExistTrxCatBudget> {
    async move {
        let mut query = sqlx::QueryBuilder::new(r#"SELECT * FROM tblcategorybudgets WHERE id = "#);

        let trx = query
            .push_bind(id)
            .build_query_as::<ExistTrxCatBudget>()
            .fetch_one(db.as_executor())
            .await
            .unwrap();

        trx
    }
    .boxed()
}

pub fn query_latest_trx_cat_budget_by_catid<'a>(
    db: &'a mut impl Executor,
    id: i32
) -> BoxFuture<'a, Option<ExistTrxCatBudget>> {
    async move {
        let mut query = sqlx::QueryBuilder::new(r#"SELECT * FROM tblcategorybudgets WHERE categoryid = "#);
        query
            .push_bind(id)
            .push(" ORDER By id DESC")
            .push(" LIMIT 1");

        let result = query
            .build_query_as::<ExistTrxCatBudget>()
            .fetch_one(db.as_executor())
            .await;

        match result {
            Err(sqlx::Error::RowNotFound) => None,
            _ => Some(result.unwrap()),
        }
    }
    .boxed()
}

pub fn query_add_trx_cat_budget<'a>(
    db: &'a mut impl Executor,
    trx_cat_budget: NewTrxCatBudget,
) -> BoxFuture<'a, MySqlQueryResult> {
    async move {

        let values = vec![
            trx_cat_budget.periode.to_string(),
            trx_cat_budget.allocated.to_string(),
            trx_cat_budget.spent.to_string(),
            trx_cat_budget.available.to_string(),
            trx_cat_budget.categoryid.to_string(),
        ];

        let mut query = sqlx::QueryBuilder::new(r#"INSERT INTO tblcategorybudgets (periode, allocated, spent, available, categoryid) VALUES ("#);

        let mut separated = query.separated(", ");
        for value in values.iter() {
            separated.push_bind(value);
        }
        separated.push_unseparated(") ");

        let add = query
            .build()
            .execute(db.as_executor())
            .await
            .unwrap();

        add
    }
    .boxed()
}

pub fn query_update_trx_cat_budget<'a>(
    db: &'a mut impl Executor,
    id: i32,
    trx_cat_budget: UpdateTrxCatBudget,
) -> BoxFuture<'a, MySqlQueryResult> {
    async move {

        let mut query = sqlx::QueryBuilder::new(r#"UPDATE tblcategorybudgets SET "#);
        let mut updates: Vec<UpdateQuery> = Vec::new();

        if trx_cat_budget.periode.is_some() {
            updates.push(UpdateQuery {
                key: "periode".to_string(),
                value: trx_cat_budget.periode.unwrap().to_string(),
            })
        }

        if trx_cat_budget.allocated.is_some() {
            updates.push(UpdateQuery {
                key: "allocated".to_string(),
                value: trx_cat_budget.allocated.unwrap().to_string(),
            })
        }

        if trx_cat_budget.available.is_some() {
            updates.push(UpdateQuery {
                key: "available".to_string(),
                value: trx_cat_budget.available.unwrap().to_string(),
            })
        }

        if trx_cat_budget.categoryid.is_some() {
            updates.push(UpdateQuery {
                key: "categoryid".to_string(),
                value: trx_cat_budget.categoryid.unwrap().to_string(),
            })
        }

        let mut separated = query.separated(", ");
        for update in updates.iter() {
            separated.push(update.key.clone())
                .push_unseparated(" = ")
                .push_bind_unseparated(update.value.clone());
        }

        separated.push_unseparated(" WHERE id = ")
            .push_bind_unseparated(id);
        
        let res = query.build()
            .execute(db.as_executor())
            .await
            .unwrap();

        res
    }
    .boxed()
}

pub fn query_delete_cat_budget<'a>(
    db: &'a mut impl Executor,
    id: i32,
) -> BoxFuture<'a, MySqlQueryResult> {
    async move {

        let mut query = sqlx::QueryBuilder::new(r#"DELETE FROM tblcategorybudgets WHERE id = "#);
        let res = query.push_bind(id)
            .build()
            .execute(db.as_executor())
            .await
            .unwrap();

        res
    }
    .boxed()
}

pub fn query_delete_cat_budget_by_catid<'a>(
    db: &'a mut impl Executor,
    categoryid: i32,
) -> BoxFuture<'a, MySqlQueryResult> {
    async move {

        let mut query = sqlx::QueryBuilder::new(r#"DELETE FROM tblcategorybudgets WHERE categoryid = "#);
        let res = query
            .push_bind(categoryid)
            .build()
            .execute(db.as_executor())
            .await
            .unwrap();

        res
    }
    .boxed()
}