
use crate::models::trx_cats::{ ExistTrxCat, NewTrxCat, UpdateTrxCat };
use crate::repositories::Executor;

use futures_util::{future::BoxFuture, FutureExt};
use sqlx::{MySql, MySqlPool};

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
    ) -> Result<Vec<ExistTrxCat>, Box<dyn std::error::Error + Send + Sync + 'static>>;
    async fn trx_cats_detail(
        &mut self,
        id: i32,
    ) -> Result<ExistTrxCat, Box<dyn std::error::Error + Send + Sync + 'static>>;
    async fn trx_cats_add(
        &mut self,
        account: NewTrxCat,
    ) -> Result<ExistTrxCat, Box<dyn std::error::Error + Send + Sync + 'static>>;
    async fn trx_cats_update(
        &mut self,
        id: i32,
        account: UpdateTrxCat,
    ) -> Result<ExistTrxCat, Box<dyn std::error::Error + Send + Sync + 'static>>;
    async fn trx_cats_delete(
        &mut self,
        id: i32,
    ) -> Result<ExistTrxCat, Box<dyn std::error::Error + Send + Sync + 'static>>;
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
    ) -> Result<Vec<ExistTrxCat>, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let trx_cats = query_list_trx_cats(&mut self.db).await;

        Ok(trx_cats)
    }

    async fn trx_cats_detail(
        &mut self,
        id: i32,
    ) -> Result<ExistTrxCat, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let trx_cat = query_detail_trx_cats(&mut self.db, id).await;

        Ok(trx_cat)
    }

    async fn trx_cats_add(
        &mut self,
        cat: NewTrxCat,
    ) -> Result<ExistTrxCat, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let trx_cat = query_add_trx_cats(&mut self.db, cat).await;

        Ok(trx_cat)
    }

    async fn trx_cats_update(
        &mut self,
        id: i32,
        cat: UpdateTrxCat,
    ) -> Result<ExistTrxCat, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let trx_cat = query_update_trx_cats(&mut self.db, id, cat).await;

        Ok(trx_cat)
    }

    async fn trx_cats_delete(
        &mut self,
        id: i32,
    ) -> Result<ExistTrxCat, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let trx_cat = query_delete_trx_cats(&mut self.db, id).await;

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
    cat: NewTrxCat,
) -> BoxFuture<'a, ExistTrxCat> {
    async move {

        let cat_name = cat.name;
        let cat_desc: String = match cat.description { 
            Some(_) => cat.description.unwrap().to_string(),
            None => "".to_string()
        };
        let values = vec![cat_name, cat_desc];
        let mut query = sqlx::QueryBuilder::new(r#"INSERT INTO tbltransactioncategories (name, description) VALUES ("#);

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

        let mut query = sqlx::QueryBuilder::new(r#"SELECT * FROM tbltransactioncategories WHERE id = "#);
        let trx_cat = query
            .push_bind(add.last_insert_id())
            .build_query_as::<ExistTrxCat>()
            .fetch_one(db.as_executor())
            .await
            .unwrap();

        trx_cat
    }
    .boxed()
}

struct UpdateQuery {
    key: String,
    value: String,
}

fn query_update_trx_cats<'a>(
    db: &'a mut impl Executor,
    id: i32,
    cat: UpdateTrxCat,
) -> BoxFuture<'a, ExistTrxCat> {
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

        separated.push_unseparated(" WHERE id = ")
            .push_bind_unseparated(id);
        
        query.build()
            .execute(db.as_executor())
            .await
            .unwrap();

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

fn query_delete_trx_cats<'a>(
    db: &'a mut impl Executor,
    id: i32,
) -> BoxFuture<'a, ExistTrxCat> {
    async move {

        let mut query = sqlx::QueryBuilder::new(r#"SELECT * FROM tbltransactioncategories WHERE id = "#);
        let trx_cats = query
            .push_bind(id)
            .build_query_as::<ExistTrxCat>()
            .fetch_one(db.as_executor())
            .await
            .unwrap();

        let mut query = sqlx::QueryBuilder::new(r#"DELETE FROM tbltransactioncategories WHERE id = "#);
        query.push_bind(id)
            .build()
            .execute(db.as_executor())
            .await
            .unwrap();

        trx_cats
    }
    .boxed()
}