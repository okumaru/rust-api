
use crate::models::bigdecimal_to_int;
use crate::models::trxs::{ ExistTrx, ExistTrxWithAccCat, NewTrx, AddTrx, UpdateTrx };
use crate::repositories::{ Executor, UpdateQuery };
use crate::repositories::accounts;
use crate::repositories::trx_cats;
use crate::repositories::trx_cat_budgets;

use futures_util::{future::BoxFuture, FutureExt};
use sqlx::{MySql, MySqlPool};
use sqlx_mysql::MySqlQueryResult;

#[async_trait::async_trait]
pub trait Trait: Send + Sync + TrxTrait {
    fn clone_boxed(&self) -> Box<dyn Trait>;
    async fn start_transaction(
        &self,
    ) -> Result<Box<dyn TransactionTrait>, Box<dyn std::error::Error + Send + Sync + 'static>>;
}

#[async_trait::async_trait]
pub trait TransactionTrait: Send + Sync + TrxTrait {
    async fn commit(
        self: Box<Self>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>>;
}

#[async_trait::async_trait]
pub trait TrxTrait {
    async fn trxs_list(
        &mut self,
        accountid: Option<String>,
        categoryid: Option<String>
    ) -> Result<Vec<ExistTrxWithAccCat>, Box<dyn std::error::Error + Send + Sync + 'static>>;
    async fn trx_detail(
        &mut self,
        id: i32,
    ) -> Result<ExistTrxWithAccCat, Box<dyn std::error::Error + Send + Sync + 'static>>;
    async fn trx_add(
        &mut self,
        account: NewTrx,
    ) -> Result<ExistTrx, Box<dyn std::error::Error + Send + Sync + 'static>>;
    async fn trx_update(
        &mut self,
        id: i32,
        account: UpdateTrx,
    ) -> Result<ExistTrx, Box<dyn std::error::Error + Send + Sync + 'static>>;
    async fn trx_delete(
        &mut self,
        id: i32,
    ) -> Result<ExistTrx, Box<dyn std::error::Error + Send + Sync + 'static>>;
}

#[derive(Debug, Clone)]
pub struct TrxRepo<E = MySqlPool> {
    db: E,
}

impl TrxRepo {
    pub fn new(pool: MySqlPool) -> Self {
        Self { db: pool }
    }
}

#[async_trait::async_trait]
impl Trait for TrxRepo {
    fn clone_boxed(&self) -> Box<dyn Trait> {
        Box::new(Clone::clone(self))
    }

    async fn start_transaction(
        &self,
    ) -> Result<Box<dyn TransactionTrait>, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let tx = self.db.begin().await?;

        Ok(Box::new(TrxRepo { db: tx }))
    }
}

#[async_trait::async_trait]
impl TransactionTrait for TrxRepo<sqlx::Transaction<'static, MySql>> {
    async fn commit(
        self: Box<Self>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.db.commit().await?;

        Ok(())
    }
}

#[async_trait::async_trait]
impl<E: 'static + Executor> TrxTrait for TrxRepo<E> {
    async fn trxs_list(
        &mut self,
        accountid: Option<String>,
        categoryid: Option<String>
    ) -> Result<Vec<ExistTrxWithAccCat>, Box<dyn std::error::Error + Send + Sync + 'static>> {

        let mut trxs: Vec<ExistTrxWithAccCat> = Vec::new();

        let data_trxs = query_list_trx(&mut self.db, accountid, categoryid).await;
        for data in data_trxs.iter() {

            let acc_id = data.accountid;
            let cat_id = data.categoryid;

            let account = accounts::query_detail_account(&mut self.db, acc_id).await;
            let category = trx_cats::query_detail_trx_cats(&mut self.db, cat_id).await;

            let trx: ExistTrxWithAccCat = ExistTrxWithAccCat {
                id: data.id,
                credit: data.credit.clone(),
                debit: data.debit.clone(),
                description: data.description.clone(),
                balance_before: data.balance_before.clone(),
                balance_after: data.balance_after.clone(),
                datetime: data.datetime,
                created_at: data.created_at,
                updated_at: data.updated_at,
                accountid: data.accountid,
                categoryid: data.categoryid,
                account: account,
                category: category
            };

            trxs.push(trx);
        }

        Ok(trxs)
    }

    async fn trx_detail(
        &mut self,
        id: i32,
    ) -> Result<ExistTrxWithAccCat, Box<dyn std::error::Error + Send + Sync + 'static>> {

        let data_trx = query_detail_trx(&mut self.db, id).await;

        let acc_id = data_trx.accountid;
        let cat_id = data_trx.categoryid;

        let account = accounts::query_detail_account(&mut self.db, acc_id).await;
        let category = trx_cats::query_detail_trx_cats(&mut self.db, cat_id).await;

        let trx: ExistTrxWithAccCat = ExistTrxWithAccCat {
            id: data_trx.id,
            credit: data_trx.credit,
            debit: data_trx.debit,
            description: data_trx.description,
            balance_before: data_trx.balance_before,
            balance_after: data_trx.balance_after,
            datetime: data_trx.datetime,
            created_at: data_trx.created_at,
            updated_at: data_trx.updated_at,
            accountid: data_trx.accountid,
            categoryid: data_trx.categoryid,
            account: account,
            category: category
        };

        Ok(trx)
    }

    async fn trx_add(
        &mut self,
        trx: NewTrx,
    ) -> Result<ExistTrx, Box<dyn std::error::Error + Send + Sync + 'static>> {

        // account detail
        let account = accounts::query_detail_account(&mut self.db, trx.accountid).await;
        let acc_balance = bigdecimal_to_int(account.balance);
        let amount = trx.credit - trx.debit;
        let acc_id = trx.accountid;
        let cat_id = trx.categoryid;

        let add_trx = AddTrx {
            credit: trx.credit,
            debit: trx.debit,
            description: trx.description,
            balance_before: acc_balance,
            balance_after: &acc_balance + &amount,
            datetime: trx.datetime,
            accountid: acc_id.clone(),
            categoryid: cat_id.clone(),
        };

        // add trx
        let add = query_add_trx(&mut self.db, add_trx).await;
        let trx_id = i32::try_from(add.last_insert_id()).unwrap();

        // update account balance
        let _ = accounts::update_acc_balance(&mut self.db, acc_id, amount).await;

        // update trx cat budget current periode
        let _ = trx_cat_budgets::query_update_trx_cat_badget(&mut self.db, cat_id, amount).await;

        // detail trx
        let trx = query_detail_trx(&mut self.db, trx_id).await;

        Ok(trx)
    }

    async fn trx_update(
        &mut self,
        id: i32,
        trx: UpdateTrx,
    ) -> Result<ExistTrx, Box<dyn std::error::Error + Send + Sync + 'static>> {

        // trx detail
        let exist_trx = query_detail_trx(&mut self.db, id).await;

        if trx.debit.is_none() == false && trx.credit.is_none() == false {

            let acc_id = exist_trx.accountid;
            let cat_id = exist_trx.categoryid;
            let exist_amount = bigdecimal_to_int(&exist_trx.credit - &exist_trx.debit);
            let new_amount = trx.credit.unwrap() - trx.debit.unwrap();

            if exist_amount != new_amount {

                let amount = new_amount - exist_amount;

                // update trx after & bef balance
                let _ = update_curr_trx_balance(&mut self.db, id, acc_id.clone(), amount).await;
                let _ = update_trx_balance(&mut self.db, id, acc_id.clone(), amount).await;

                // update account balance
                let _ = accounts::update_acc_balance(&mut self.db, acc_id, amount).await;

                // update trx cat budget current periode
                let _ = trx_cat_budgets::query_update_trx_cat_badget(&mut self.db, cat_id, amount).await;
            }

        }

        // update trx credit, debit, desc, categoryid
        let _ = query_update_trx(&mut self.db, id, trx).await;

        // trx detail
        let trx = query_detail_trx(&mut self.db, id).await;

        Ok(trx)
    }

    async fn trx_delete(
        &mut self,
        id: i32,
    ) -> Result<ExistTrx, Box<dyn std::error::Error + Send + Sync + 'static>> {

        // trx detail
        let trx = query_detail_trx(&mut self.db, id).await;
        let amount = bigdecimal_to_int(&trx.debit - &trx.credit);
        let acc_id = trx.accountid;
        let cat_id = trx.categoryid;

        // update trx after & bef balance
        let _ = update_trx_balance(&mut self.db, id, acc_id.clone(), amount).await;

        // update account balance
        let _ = accounts::update_acc_balance(&mut self.db, acc_id, amount).await;

        // update trx cat budget current periode
        let _ = trx_cat_budgets::query_update_trx_cat_badget(&mut self.db, cat_id, amount).await;

        // delete trx
        let _ = query_delete_trx(&mut self.db, id).await;

        Ok(trx)
    }
}

fn query_list_trx<'a>(
    db: &'a mut impl Executor,
    accountid: Option<String>,
    categoryid: Option<String>
) -> BoxFuture<'a, Vec<ExistTrx>> {
    async move {
        let mut query = sqlx::QueryBuilder::new(r#"SELECT * FROM tbltransactions"#);

        if accountid.is_some() || categoryid.is_some() {
            query.push(" WHERE ");
        }

        let mut conditions: Vec<UpdateQuery> = Vec::new();

        if accountid.is_some() {
            conditions.push(UpdateQuery {
                key: String::from("accountid"),
                value: accountid.unwrap(),
            });
        }

        if categoryid.is_some() {
            conditions.push(UpdateQuery {
                key: String::from("categoryid"),
                value: categoryid.unwrap(),
            });
        }

        let mut separated = query.separated(" AND ");
        for condition in conditions.iter() {
            separated.push(condition.key.clone())
                .push_unseparated(" = ")
                .push_bind_unseparated(condition.value.clone());
        }

        separated.push_unseparated(" ORDER by datetime DESC");

        let trxs = query
            .build_query_as::<ExistTrx>()
            .fetch_all(db.as_executor())
            .await
            .unwrap();

        trxs
    }
    .boxed()
}

fn query_detail_trx<'a>(
    db: &'a mut impl Executor,
    id: i32
) -> BoxFuture<'a, ExistTrx> {
    async move {
        let mut query = sqlx::QueryBuilder::new(r#"SELECT * FROM tbltransactions WHERE id = "#);

        let trx = query
            .push_bind(id)
            .build_query_as::<ExistTrx>()
            .fetch_one(db.as_executor())
            .await
            .unwrap();

        trx
    }
    .boxed()
}

fn query_add_trx<'a>(
    db: &'a mut impl Executor,
    trx: AddTrx,
) -> BoxFuture<'a, MySqlQueryResult> {
    async move {

        let desc: String = match trx.description { 
            Some(_) => trx.description.unwrap().to_string(),
            None => "".to_string()
        };

        let values = vec![
            trx.credit.to_string(),
            trx.debit.to_string(),
            desc,
            trx.balance_before.to_string(),
            trx.balance_after.to_string(),
            trx.datetime.to_string(),
            trx.accountid.to_string(),
            trx.categoryid.to_string(),
        ];

        let mut query = sqlx::QueryBuilder::new(r#"INSERT INTO tbltransactions (credit, debit, description, balance_before, balance_after, datetime, accountid, categoryid) VALUES ("#);

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

fn update_trx_balance<'a>(
    db: &'a mut impl Executor,
    id: i32,
    acc_id: i32,
    amount: i64,
) -> BoxFuture<'a, MySqlQueryResult> {
    async move {

        let mut query = sqlx::QueryBuilder::new(r#"UPDATE tbltransactions SET "#);
        query.push("balance_after = balance_after + ").push_bind(amount)
            .push(" , balance_before = balance_before + ").push_bind(amount)
            .push(" , updated_at = current_timestamp() ")
            .push(" WHERE accountid = ").push_bind(acc_id)
            .push(" AND id > ").push_bind(id);

        let res = query
            .build()
            .execute(db.as_executor())
            .await
            .unwrap();

        res
    }
    .boxed()
}

fn update_curr_trx_balance<'a>(
    db: &'a mut impl Executor,
    id: i32,
    acc_id: i32,
    amount: i64,
) -> BoxFuture<'a, MySqlQueryResult> {
    async move {

        let mut query = sqlx::QueryBuilder::new(r#"UPDATE tbltransactions SET "#);
        query.push("balance_after = balance_after + ").push_bind(amount)
            .push(" , updated_at = current_timestamp() ")
            .push(" WHERE accountid = ").push_bind(acc_id)
            .push(" AND id = ").push_bind(id);

        let res = query
            .build()
            .execute(db.as_executor())
            .await
            .unwrap();

        res
    }
    .boxed()
}

fn query_update_trx<'a>(
    db: &'a mut impl Executor,
    id: i32,
    trx: UpdateTrx,
) -> BoxFuture<'a, MySqlQueryResult> {
    async move {

        let mut query = sqlx::QueryBuilder::new(r#"UPDATE tbltransactions SET "#);
        let mut updates: Vec<UpdateQuery> = Vec::new();

        if trx.credit.is_some() {
            updates.push(UpdateQuery {
                key: "credit".to_string(),
                value: trx.credit.unwrap().to_string(),
            })
        }

        if trx.debit.is_some() {
            updates.push(UpdateQuery {
                key: "debit".to_string(),
                value: trx.debit.unwrap().to_string(),
            })
        }

        if trx.description.is_some() {
            updates.push(UpdateQuery {
                key: "description".to_string(),
                value: trx.description.unwrap().to_string(),
            })
        }

        if trx.datetime.is_some() {
            updates.push(UpdateQuery {
                key: "datetime".to_string(),
                value: trx.datetime.unwrap().to_string(),
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

fn query_delete_trx<'a>(
    db: &'a mut impl Executor,
    id: i32,
) -> BoxFuture<'a, MySqlQueryResult> {
    async move {

        let mut query = sqlx::QueryBuilder::new(r#"DELETE FROM tbltransactions WHERE id = "#);
        let res = query.push_bind(id)
            .build()
            .execute(db.as_executor())
            .await
            .unwrap();

        res
    }
    .boxed()
}