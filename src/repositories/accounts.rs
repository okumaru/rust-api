
use crate::models::accounts::{ ExistAccount, NewAccount, UpdateAccount };
use crate::repositories::{ Executor, UpdateQuery };

use futures_util::{future::BoxFuture, FutureExt};
use sqlx::{MySql, MySqlPool};

#[async_trait::async_trait]
pub trait Trait: Send + Sync + AccountTrait {
    fn clone_boxed(&self) -> Box<dyn Trait>;
    async fn start_transaction(
        &self,
    ) -> Result<Box<dyn TransactionTrait>, Box<dyn std::error::Error + Send + Sync + 'static>>;
}

#[async_trait::async_trait]
pub trait TransactionTrait: Send + Sync + AccountTrait {
    async fn commit(
        self: Box<Self>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>>;
}

#[async_trait::async_trait]
pub trait AccountTrait {
    async fn account_list(
        &mut self,
    ) -> Result<Vec<ExistAccount>, Box<dyn std::error::Error + Send + Sync + 'static>>;
    async fn account_detail(
        &mut self,
        id: i32,
    ) -> Result<ExistAccount, Box<dyn std::error::Error + Send + Sync + 'static>>;
    async fn account_add(
        &mut self,
        account: NewAccount,
    ) -> Result<ExistAccount, Box<dyn std::error::Error + Send + Sync + 'static>>;
    async fn account_update(
        &mut self,
        id: i32,
        account: UpdateAccount,
    ) -> Result<ExistAccount, Box<dyn std::error::Error + Send + Sync + 'static>>;
    async fn account_delete(
        &mut self,
        id: i32,
    ) -> Result<ExistAccount, Box<dyn std::error::Error + Send + Sync + 'static>>;
}

#[derive(Debug, Clone)]
pub struct AccountRepo<E = MySqlPool> {
    db: E,
}

impl AccountRepo {
    pub fn new(pool: MySqlPool) -> Self {
        Self { db: pool }
    }
}

#[async_trait::async_trait]
impl Trait for AccountRepo {
    fn clone_boxed(&self) -> Box<dyn Trait> {
        Box::new(Clone::clone(self))
    }

    async fn start_transaction(
        &self,
    ) -> Result<Box<dyn TransactionTrait>, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let tx = self.db.begin().await?;

        Ok(Box::new(AccountRepo { db: tx }))
    }
}

#[async_trait::async_trait]
impl TransactionTrait for AccountRepo<sqlx::Transaction<'static, MySql>> {
    async fn commit(
        self: Box<Self>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.db.commit().await?;

        Ok(())
    }
}

#[async_trait::async_trait]
impl<E: 'static + Executor> AccountTrait for AccountRepo<E> {
    async fn account_list(
        &mut self,
    ) -> Result<Vec<ExistAccount>, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let accounts = query_list_accounts(&mut self.db).await;

        Ok(accounts)
    }

    async fn account_detail(
        &mut self,
        id: i32,
    ) -> Result<ExistAccount, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let account = query_detail_account(&mut self.db, id).await;

        Ok(account)
    }

    async fn account_add(
        &mut self,
        account: NewAccount,
    ) -> Result<ExistAccount, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let account = query_add_account(&mut self.db, account).await;

        Ok(account)
    }

    async fn account_update(
        &mut self,
        id: i32,
        account: UpdateAccount,
    ) -> Result<ExistAccount, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let account = query_update_account(&mut self.db, id, account).await;

        Ok(account)
    }

    async fn account_delete(
        &mut self,
        id: i32,
    ) -> Result<ExistAccount, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let account = query_delete_account(&mut self.db, id).await;

        Ok(account)
    }
}

pub fn query_list_accounts<'a>(
    db: &'a mut impl Executor,
) -> BoxFuture<'a, Vec<ExistAccount>> {
    async move {
        let mut query = sqlx::QueryBuilder::new(r#"SELECT * FROM tblaccounts"#);

        let accounts = query
            .build_query_as::<ExistAccount>()
            .fetch_all(db.as_executor())
            .await
            .unwrap();

        accounts
    }
    .boxed()
}

pub fn query_detail_account<'a>(
    db: &'a mut impl Executor,
    id: i32
) -> BoxFuture<'a, ExistAccount> {
    async move {
        let mut query = sqlx::QueryBuilder::new(r#"SELECT * FROM tblaccounts WHERE id = "#);

        let accounts = query
            .push_bind(id)
            .build_query_as::<ExistAccount>()
            .fetch_one(db.as_executor())
            .await
            .unwrap();

        accounts
    }
    .boxed()
}

pub fn query_add_account<'a>(
    db: &'a mut impl Executor,
    account: NewAccount,
) -> BoxFuture<'a, ExistAccount> {
    async move {

        let account_name = account.name;
        let account_balance = account.balance.to_string();
        let account_star = Into::<i32>::into(account.star).to_string();
        let account_type = account.r#type.to_string();
        let account_desc: String = match account.description { 
            Some(_) => account.description.unwrap().to_string(),
            None => "".to_string()
        };
        let values = vec![account_name, account_desc, account_star, account_type, account_balance];
        let mut query = sqlx::QueryBuilder::new(r#"INSERT INTO tblaccounts (name, description, star, type, balance) VALUES ("#);

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

        let mut query = sqlx::QueryBuilder::new(r#"SELECT * FROM tblaccounts WHERE id = "#);
        let accounts = query
            .push_bind(add.last_insert_id())
            .build_query_as::<ExistAccount>()
            .fetch_one(db.as_executor())
            .await
            .unwrap();

        accounts
    }
    .boxed()
}

pub fn query_update_account<'a>(
    db: &'a mut impl Executor,
    id: i32,
    account: UpdateAccount,
) -> BoxFuture<'a, ExistAccount> {
    async move {

        let mut query = sqlx::QueryBuilder::new(r#"UPDATE tblaccounts SET "#);
        let mut updates: Vec<UpdateQuery> = Vec::new();

        if account.name.is_some() {
            updates.push(UpdateQuery {
                key: "name".to_string(),
                value: account.name.unwrap().to_string(),
            })
        }

        if account.description.is_some() {
            updates.push(UpdateQuery {
                key: "description".to_string(),
                value: account.description.unwrap().to_string(),
            })
        }

        if account.star.is_some() {
            updates.push(UpdateQuery {
                key: "star".to_string(),
                value: Into::<i32>::into(account.star.unwrap()).to_string(),
            })
        }

        if account.r#type.is_some() {
            updates.push(UpdateQuery {
                key: "type".to_string(),
                value: account.r#type.unwrap().to_string(),
            })
        }

        if account.balance.is_some() {
            updates.push(UpdateQuery {
                key: "balance".to_string(),
                value: account.balance.unwrap().to_string(),
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

        let mut query = sqlx::QueryBuilder::new(r#"SELECT * FROM tblaccounts WHERE id = "#);
        let accounts = query
            .push_bind(id)
            .build_query_as::<ExistAccount>()
            .fetch_one(db.as_executor())
            .await
            .unwrap();

        accounts
    }
    .boxed()
}

pub fn query_delete_account<'a>(
    db: &'a mut impl Executor,
    id: i32,
) -> BoxFuture<'a, ExistAccount> {
    async move {

        let mut query = sqlx::QueryBuilder::new(r#"SELECT * FROM tblaccounts WHERE id = "#);
        let accounts = query
            .push_bind(id)
            .build_query_as::<ExistAccount>()
            .fetch_one(db.as_executor())
            .await
            .unwrap();

        let mut query = sqlx::QueryBuilder::new(r#"DELETE FROM tblaccounts WHERE id = "#);
        query.push_bind(id)
            .build()
            .execute(db.as_executor())
            .await
            .unwrap();

        accounts
    }
    .boxed()
}