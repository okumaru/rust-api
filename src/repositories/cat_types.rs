
use crate::models::cat_types::{ ExistCatType, ExistCatTypeWithBudget, AddCatType, UpdateCatType };
use crate::repositories::{ Executor, UpdateQuery };

use futures_util::{future::BoxFuture, FutureExt};
use sqlx::{MySql, MySqlPool};
use sqlx_mysql::MySqlQueryResult;

#[async_trait::async_trait]
pub trait Trait: Send + Sync + CatTypeTrait {
    fn clone_boxed(&self) -> Box<dyn Trait>;
    async fn start_transaction(
        &self,
    ) -> Result<Box<dyn TransactionTrait>, Box<dyn std::error::Error + Send + Sync + 'static>>;
}

#[async_trait::async_trait]
pub trait TransactionTrait: Send + Sync + CatTypeTrait {
    async fn commit(
        self: Box<Self>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>>;
}

#[async_trait::async_trait]
pub trait CatTypeTrait {

    async fn cat_types_list(
        &mut self,
    ) -> Result<Vec<ExistCatTypeWithBudget>, Box<dyn std::error::Error + Send + Sync + 'static>>;

    async fn cat_type_detail(
        &mut self,
        id: i32,
    ) -> Result<ExistCatTypeWithBudget, Box<dyn std::error::Error + Send + Sync + 'static>>;

    async fn cat_type_add(
        &mut self,
        cat_type: AddCatType,
    ) -> Result<ExistCatTypeWithBudget, Box<dyn std::error::Error + Send + Sync + 'static>>;

    async fn cat_type_update(
        &mut self,
        id: i32,
        cat_type: UpdateCatType,
    ) -> Result<ExistCatTypeWithBudget, Box<dyn std::error::Error + Send + Sync + 'static>>;

    async fn cat_type_delete(
        &mut self,
        id: i32,
    ) -> Result<ExistCatTypeWithBudget, Box<dyn std::error::Error + Send + Sync + 'static>>;
}

#[derive(Debug, Clone)]
pub struct CatTypeRepo<E = MySqlPool> {
    db: E,
}

impl CatTypeRepo {
    pub fn new(pool: MySqlPool) -> Self {
        Self { db: pool }
    }
}

#[async_trait::async_trait]
impl Trait for CatTypeRepo {
    fn clone_boxed(&self) -> Box<dyn Trait> {
        Box::new(Clone::clone(self))
    }

    async fn start_transaction(
        &self,
    ) -> Result<Box<dyn TransactionTrait>, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let tx = self.db.begin().await?;

        Ok(Box::new(CatTypeRepo { db: tx }))
    }
}

#[async_trait::async_trait]
impl TransactionTrait for CatTypeRepo<sqlx::Transaction<'static, MySql>> {
    async fn commit(
        self: Box<Self>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.db.commit().await?;

        Ok(())
    }
}

#[async_trait::async_trait]
impl<E: 'static + Executor> CatTypeTrait for CatTypeRepo<E> {

    async fn cat_types_list(
        &mut self,
    ) -> Result<Vec<ExistCatTypeWithBudget>, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let cat_types = query_list_cat_types(&mut self.db).await;
        Ok(cat_types)
    }

    async fn cat_type_detail(
        &mut self,
        id: i32,
    ) -> Result<ExistCatTypeWithBudget, Box<dyn std::error::Error + Send + Sync + 'static>> {
        // detail cat type
        let cat_types = query_detail_cat_type(&mut self.db, id).await;
        Ok(cat_types)
    }

    async fn cat_type_add(
        &mut self,
        cat_type: AddCatType,
    ) -> Result<ExistCatTypeWithBudget, Box<dyn std::error::Error + Send + Sync + 'static>> {
        // add cat type
        let add = query_add_cat_type(&mut self.db, cat_type).await;
        let cat_type_id = i32::try_from(add.last_insert_id()).unwrap();

        // detail cat type
        let cat_type = query_detail_cat_type(&mut self.db, cat_type_id).await;
        Ok(cat_type)
    }

    async fn cat_type_update(
        &mut self,
        id: i32,
        cat_type: UpdateCatType,
    ) -> Result<ExistCatTypeWithBudget, Box<dyn std::error::Error + Send + Sync + 'static>> {
        // update cat type
        let _ = query_update_cat_type(&mut self.db, id, cat_type).await;

        // cat type detail
        let cat_type = query_detail_cat_type(&mut self.db, id).await;
        Ok(cat_type)
    }

    async fn cat_type_delete(
        &mut self,
        id: i32,
    ) -> Result<ExistCatTypeWithBudget, Box<dyn std::error::Error + Send + Sync + 'static>> {
        // cat type detail
        let cat_type = query_detail_cat_type(&mut self.db, id).await;
        // delete cat type
        let _ = query_delete_cat_type(&mut self.db, id).await;
        Ok(cat_type)
    }
}

fn query_list_cat_types<'a>(
    db: &'a mut impl Executor,
) -> BoxFuture<'a, Vec<ExistCatTypeWithBudget>> {
    async move {
        let mut query = sqlx::QueryBuilder::new(r#"SELECT 
            t.*, SUM(t3.allocated) as allocated, SUM(t3.spent) as spent, SUM(t3.available) as available
            FROM tblcategorytypes t 
            LEFT JOIN tbltransactioncategories t2 ON t2.typeid = t.id 
            LEFT JOIN tblcategorybudgets t3 ON t3.categoryid = t2.id AND t3.id = (
                SELECT MAX(budget.id) from tblcategorybudgets budget WHERE budget.categoryid = t2.id
            )
            GROUP by t.id 
            ORDER by t.id ASC"#);

        let cat_types = query
            .build_query_as::<ExistCatTypeWithBudget>()
            .fetch_all(db.as_executor())
            .await
            .unwrap();

        cat_types
    }
    .boxed()
}

fn query_detail_cat_type<'a>(
    db: &'a mut impl Executor,
    id: i32
) -> BoxFuture<'a, ExistCatTypeWithBudget> {
    async move {
        let mut query = sqlx::QueryBuilder::new(r#"SELECT t.*, SUM(t3.allocated) as allocated, SUM(t3.spent) as spent, SUM(t3.available) as available
            FROM tblcategorytypes t 
            LEFT JOIN tbltransactioncategories t2 ON t2.typeid = t.id 
            LEFT JOIN tblcategorybudgets t3 ON t3.categoryid = t2.id AND t3.id = (SELECT MAX(budget.id) from tblcategorybudgets budget WHERE budget.categoryid = t2.id)
            WHERE t.id = "#);

        let cat_type = query
            .push_bind(id)
            .build_query_as::<ExistCatTypeWithBudget>()
            .fetch_one(db.as_executor())
            .await
            .unwrap();

        cat_type
    }
    .boxed()
}

fn query_add_cat_type<'a>(
    db: &'a mut impl Executor,
    cat_type: AddCatType,
) -> BoxFuture<'a, MySqlQueryResult> {
    async move {

        let desc: String = match cat_type.description { 
            Some(_) => cat_type.description.unwrap().to_string(),
            None => "".to_string()
        };

        let values = vec![
            cat_type.r#type.to_string(),
            desc,
            cat_type.icon.to_string(),
        ];

        let mut query = sqlx::QueryBuilder::new(r#"INSERT INTO tblcategorytypes (type, description, icon) VALUES ("#);

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

fn query_update_cat_type<'a>(
    db: &'a mut impl Executor,
    id: i32,
    cat_type: UpdateCatType,
) -> BoxFuture<'a, MySqlQueryResult> {
    async move {

        let mut query = sqlx::QueryBuilder::new(r#"UPDATE tblcategorytypes SET "#);
        let mut updates: Vec<UpdateQuery> = Vec::new();

        if cat_type.r#type.is_some() {
            updates.push(UpdateQuery {
                key: "type".to_string(),
                value: cat_type.r#type.unwrap().to_string(),
            })
        }

        if cat_type.description.is_some() {
            updates.push(UpdateQuery {
                key: "description".to_string(),
                value: cat_type.description.unwrap().to_string(),
            })
        }

        if cat_type.icon.is_some() {
            updates.push(UpdateQuery {
                key: "icon".to_string(),
                value: cat_type.icon.unwrap().to_string(),
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
        
        let res = query.build()
            .execute(db.as_executor())
            .await
            .unwrap();

        res
    }
    .boxed()
}

fn query_delete_cat_type<'a>(
    db: &'a mut impl Executor,
    id: i32,
) -> BoxFuture<'a, MySqlQueryResult> {
    async move {

        let mut query = sqlx::QueryBuilder::new(r#"DELETE FROM tblcategorytypes WHERE id = "#);
        let res = query.push_bind(id)
            .build()
            .execute(db.as_executor())
            .await
            .unwrap();

        res
    }
    .boxed()
}