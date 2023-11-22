
use crate::models::trx_cats;
use crate::models::trx_cats::{ ExistTrxCat, ExistTrxCatWithBudget, AddTrxCat, UpdateTrxCat };
use crate::models::trx_cat_budgets::{ ExistTrxCatBudget, AddTrxCatBudget, NewTrxCatBudget, UpdateTrxCatBudget };
use crate::repositories::{ Executor, UpdateQuery };
use crate::repositories::trx_cat_budgets;

use futures_util::{future::BoxFuture, FutureExt};
use sqlx::{MySql, MySqlPool};
use sqlx_mysql::MySqlQueryResult;

#[async_trait::async_trait]
pub trait Trait: Send + Sync + TrxCatTrait {
    fn clone_boxed(&self) -> Box<dyn Trait>;
    async fn start_transaction(
        &self,
    ) -> Result<Box<dyn TransactionTrait>, Box<dyn std::error::Error + Send + Sync + 'static>>;
}

#[async_trait::async_trait]
pub trait TransactionTrait: Send + Sync + TrxCatTrait {
    async fn commit(
        self: Box<Self>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>>;
}

#[async_trait::async_trait]
pub trait TrxCatTrait {
    async fn trx_cats_list(
        &mut self,
    ) -> Result<Vec<ExistTrxCatWithBudget>, Box<dyn std::error::Error + Send + Sync + 'static>>;
    async fn trx_cats_detail(
        &mut self,
        id: i32,
    ) -> Result<ExistTrxCatWithBudget, Box<dyn std::error::Error + Send + Sync + 'static>>;
    async fn trx_cats_add(
        &mut self,
        account: AddTrxCat,
    ) -> Result<ExistTrxCatWithBudget, Box<dyn std::error::Error + Send + Sync + 'static>>;
    async fn trx_cats_update(
        &mut self,
        id: i32,
        account: UpdateTrxCat,
    ) -> Result<ExistTrxCatWithBudget, Box<dyn std::error::Error + Send + Sync + 'static>>;
    async fn trx_cats_delete(
        &mut self,
        id: i32,
    ) -> Result<ExistTrxCatWithBudget, Box<dyn std::error::Error + Send + Sync + 'static>>;
}

#[derive(Debug, Clone)]
pub struct TrxCatRepo<E = MySqlPool> {
    db: E,
}

impl TrxCatRepo {
    pub fn new(pool: MySqlPool) -> Self {
        Self { db: pool }
    }
}

#[async_trait::async_trait]
impl Trait for TrxCatRepo {
    fn clone_boxed(&self) -> Box<dyn Trait> {
        Box::new(Clone::clone(self))
    }

    async fn start_transaction(
        &self,
    ) -> Result<Box<dyn TransactionTrait>, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let tx = self.db.begin().await?;

        Ok(Box::new(TrxCatRepo { db: tx }))
    }
}

#[async_trait::async_trait]
impl TransactionTrait for TrxCatRepo<sqlx::Transaction<'static, MySql>> {
    async fn commit(
        self: Box<Self>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.db.commit().await?;

        Ok(())
    }
}

#[async_trait::async_trait]
impl<E: 'static + Executor> TrxCatTrait for TrxCatRepo<E> {
    async fn trx_cats_list(
        &mut self,
    ) -> Result<Vec<ExistTrxCatWithBudget>, Box<dyn std::error::Error + Send + Sync + 'static>> {

        let mut data_cats: Vec<ExistTrxCatWithBudget> = Vec::new();

        let trx_cats: Vec<ExistTrxCat> = query_list_trx_cats(&mut self.db).await;
        for cat in trx_cats.iter() {
            
            let id = cat.id;

            // detail trx cat budget
            let data_budget: Option<ExistTrxCatBudget> = trx_cat_budgets::query_latest_trx_cat_budget_by_catid(&mut self.db, id).await;
            
            let trx_cat: ExistTrxCatWithBudget = trx_cats::build_exist_trx_cat_budget(cat.clone(), data_budget);
            data_cats.push(trx_cat);

        }

        Ok(data_cats)
    }

    async fn trx_cats_detail(
        &mut self,
        id: i32,
    ) -> Result<ExistTrxCatWithBudget, Box<dyn std::error::Error + Send + Sync + 'static>> {

        // detail trx cat
        let data_cat: ExistTrxCat = query_detail_trx_cats(&mut self.db, id).await;

        // detail trx cat budget
        let data_budget: Option<ExistTrxCatBudget> = trx_cat_budgets::query_latest_trx_cat_budget_by_catid(&mut self.db, id).await;
        
        let trx_cat: ExistTrxCatWithBudget = trx_cats::build_exist_trx_cat_budget(data_cat, data_budget);
        Ok(trx_cat)
    }

    async fn trx_cats_add(
        &mut self,
        cat: AddTrxCat,
    ) -> Result<ExistTrxCatWithBudget, Box<dyn std::error::Error + Send + Sync + 'static>> {

        let mut data_budget: Option<ExistTrxCatBudget> = None;

        // add trx cat
        let add_trx_cat = query_add_trx_cats(&mut self.db, cat.clone()).await;
        let trx_cat_id = i32::try_from(add_trx_cat.last_insert_id()).unwrap();

        if cat.budget.clone() != None {

            let budget = cat.budget.clone().unwrap();
            let add_budget = NewTrxCatBudget {
                periode: budget.periode,
                allocated: budget.allocated,
                spent: budget.spent,
                available: budget.available,
                categoryid: trx_cat_id,
            };

            let _ = trx_cat_budgets::query_add_trx_cat_budget(&mut self.db, add_budget).await;
            
            // detail trx cat budget
            data_budget = trx_cat_budgets::query_latest_trx_cat_budget_by_catid(&mut self.db, trx_cat_id).await;

        }

        // detail trx cat
        let data_cat: ExistTrxCat = query_detail_trx_cats(&mut self.db, trx_cat_id).await;

        let trx_cat: ExistTrxCatWithBudget = trx_cats::build_exist_trx_cat_budget(data_cat, data_budget);
        Ok(trx_cat)
    }

    async fn trx_cats_update(
        &mut self,
        id: i32,
        cat: UpdateTrxCat,
    ) -> Result<ExistTrxCatWithBudget, Box<dyn std::error::Error + Send + Sync + 'static>> {

        // update trx cat
        let _ = query_update_trx_cats(&mut self.db, id, cat).await;

        // trx cat detail
        let data_cat = query_detail_trx_cats(&mut self.db, id).await;

        let trx_cat: ExistTrxCatWithBudget = trx_cats::build_exist_trx_cat_budget(data_cat, None);

        Ok(trx_cat)
    }

    async fn trx_cats_delete(
        &mut self,
        id: i32,
    ) -> Result<ExistTrxCatWithBudget, Box<dyn std::error::Error + Send + Sync + 'static>> {

        // trx cat detail
        let data_cat = query_detail_trx_cats(&mut self.db, id).await;

        // delete trx cat budget
        let _ = trx_cat_budgets::query_delete_cat_budget_by_catid(&mut self.db, id).await;

        // delete trx cat
        let _ = query_delete_trx_cats(&mut self.db, id).await;

        let trx_cat: ExistTrxCatWithBudget = trx_cats::build_exist_trx_cat_budget(data_cat, None);

        Ok(trx_cat)
    }
}

fn query_list_trx_cats<'a>(
    db: &'a mut impl Executor,
) -> BoxFuture<'a, Vec<ExistTrxCat>> {
    async move {
        let mut query = sqlx::QueryBuilder::new(r#"SELECT * FROM tbltransactioncategories"#);

        let trx_cats = query
            .build_query_as::<ExistTrxCat>()
            .fetch_all(db.as_executor())
            .await
            .unwrap();

        trx_cats
    }
    .boxed()
}

fn query_detail_trx_cats<'a>(
    db: &'a mut impl Executor,
    id: i32
) -> BoxFuture<'a, ExistTrxCat> {
    async move {
        let mut query = sqlx::QueryBuilder::new(r#"SELECT * FROM tbltransactioncategories WHERE id = "#);

        let trx_cat = query
            .push_bind(id)
            .build_query_as::<ExistTrxCat>()
            .fetch_one(db.as_executor())
            .await
            .unwrap();

        trx_cat
    }
    .boxed()
}

fn query_add_trx_cats<'a>(
    db: &'a mut impl Executor,
    cat: AddTrxCat,
) -> BoxFuture<'a, MySqlQueryResult> {
    async move {

        let cat_name = cat.name;
        let cat_typeid = cat.typeid.to_string();
        let cat_desc: String = match cat.description { 
            Some(_) => cat.description.unwrap().to_string(),
            None => "".to_string()
        };
        let values = vec![cat_name, cat_desc, cat_typeid];
        let mut query = sqlx::QueryBuilder::new(r#"INSERT INTO tbltransactioncategories (name, description, typeid) VALUES ("#);

        let mut separated = query.separated(", ");
        for value in values.iter() {
            separated.push_bind(value);
        }
        separated.push_unseparated(") ");

        let res = query
            .build()
            .execute(db.as_executor())
            .await
            .unwrap();

        res
    }
    .boxed()
}

fn query_update_trx_cats<'a>(
    db: &'a mut impl Executor,
    id: i32,
    cat: UpdateTrxCat,
) -> BoxFuture<'a, MySqlQueryResult> {
    async move {

        let mut query = sqlx::QueryBuilder::new(r#"UPDATE tbltransactioncategories SET "#);
        let mut updates: Vec<UpdateQuery> = Vec::new();

        if cat.name.is_some() {
            updates.push(UpdateQuery {
                key: "name".to_string(),
                value: cat.name.unwrap().to_string(),
            })
        }

        if cat.description.is_some() {
            updates.push(UpdateQuery {
                key: "description".to_string(),
                value: cat.description.unwrap().to_string(),
            })
        }

        let mut separated = query.separated(", ");
        for update in updates.iter() {
            separated.push(update.key.clone())
                .push_unseparated(" = ")
                .push_bind_unseparated(update.value.clone());
        }

        separated
            .push("updated_at = current_timestamp()")
            .push_unseparated(" WHERE id = ")
            .push_bind_unseparated(id);
        
        let res = query
            .build()
            .execute(db.as_executor())
            .await
            .unwrap();

        res
    }
    .boxed()
}

fn query_delete_trx_cats<'a>(
    db: &'a mut impl Executor,
    id: i32,
) -> BoxFuture<'a, MySqlQueryResult> {
    async move {

        let mut query = sqlx::QueryBuilder::new(r#"DELETE FROM tbltransactioncategories WHERE id = "#);

        let res = query
            .push_bind(id)
            .build()
            .execute(db.as_executor())
            .await
            .unwrap();

            res
    }
    .boxed()
}