
use crate::models::bigdecimal_to_int;
use crate::models::accounts::{ UpdateAccount };
use crate::models::trxs::{ TrxModel, ExistTrx, NewTrx, AddTrx, UpdateTrx };
use crate::repositories::{ Executor, UpdateQuery };
use crate::repositories::accounts;

use futures_util::{future::BoxFuture, FutureExt};
use sqlx::{MySql, MySqlPool};
// use sqlx::database::Database::QueryResult;
// sqlx::database::Database
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
    ) -> Result<Vec<ExistTrx>, Box<dyn std::error::Error + Send + Sync + 'static>>;
    async fn trx_detail(
        &mut self,
        id: i32,
    ) -> Result<ExistTrx, Box<dyn std::error::Error + Send + Sync + 'static>>;
    async fn trx_add(
        &mut self,
        account: NewTrx,
    ) -> Result<ExistTrx, Box<dyn std::error::Error + Send + Sync + 'static>>;
    // async fn trx_update(
    //     &mut self,
    //     id: i32,
    //     account: UpdateTrx,
    // ) -> Result<TrxModel, Box<dyn std::error::Error + Send + Sync + 'static>>;
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
    ) -> Result<Vec<ExistTrx>, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let trxs = query_list_trx(&mut self.db).await;

        Ok(trxs)
    }

    async fn trx_detail(
        &mut self,
        id: i32,
    ) -> Result<ExistTrx, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let trx = query_detail_trx(&mut self.db, id).await;

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

        let add_trx = AddTrx {
            credit: trx.credit,
            debit: trx.debit,
            description: trx.description,
            balance_before: acc_balance,
            balance_after: &acc_balance + &amount,
            accountid: acc_id.clone(),
            categoryid: trx.categoryid,
        };

        // add trx
        let add = query_add_trx(&mut self.db, add_trx).await;
        let trx_id = i32::try_from(add.last_insert_id()).unwrap();

        // update account balance
        let _ = update_acc_balance(&mut self.db, acc_id, amount).await;

        // detail trx
        let trx = query_detail_trx(&mut self.db, trx_id).await;

        Ok(trx)
    }

    // async fn trx_update(
    //     &mut self,
    //     id: i32,
    //     trx: UpdateTrx,
    // ) -> Result<ExistTrx, Box<dyn std::error::Error + Send + Sync + 'static>> {
    //     let trx = query_update_trx(&mut self.db, id, trx).await;

    //     Ok(trx)
    // }

    async fn trx_delete(
        &mut self,
        id: i32,
    ) -> Result<ExistTrx, Box<dyn std::error::Error + Send + Sync + 'static>> {

        // trx detail
        let trx = query_detail_trx(&mut self.db, id).await;
        let amount = bigdecimal_to_int(&trx.debit - &trx.credit);
        let acc_id = trx.accountid;

        // update trx after & bef balance
        let _ = update_trx_balance(&mut self.db, id, acc_id.clone(), amount).await;

        // update account balance
        let _ = update_acc_balance(&mut self.db, acc_id, amount).await;

        // delete trx
        let _ = query_delete_trx(&mut self.db, id).await;

        Ok(trx)
    }
}

fn query_list_trx<'a>(
    db: &'a mut impl Executor,
) -> BoxFuture<'a, Vec<ExistTrx>> {
    async move {
        let mut query = sqlx::QueryBuilder::new(r#"SELECT * FROM tbltransactions"#);

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
            trx.accountid.to_string(),
            trx.categoryid.to_string(),
        ];

        let mut query = sqlx::QueryBuilder::new(r#"INSERT INTO tbltransactions (credit, debit, description, balance_before, balance_after, accountid, categoryid) VALUES ("#);

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

fn update_acc_balance<'a>(
    db: &'a mut impl Executor,
    id: i32,
    amount: i64,
) -> BoxFuture<'a, MySqlQueryResult> {
    async move {

        let mut query = sqlx::QueryBuilder::new(r#"UPDATE tblaccounts SET balance = balance + "#);
        query.push_bind(amount)
            .push(" WHERE id = ")
            .push_bind(id);

        let res = query
            .build()
            .execute(db.as_executor())
            .await
            .unwrap();

        res
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
        query.push(" balance_before = balance_before + ")
            .push_bind(amount)
            .push(", balance_after = balance_after + ")
            .push_bind(amount)
            .push(" WHERE accountid = ")
            .push_bind(acc_id)
            .push(" AND id > ")
            .push_bind(id);

        let res = query
            .build()
            .execute(db.as_executor())
            .await
            .unwrap();

        res
    }
    .boxed()
}

// fn query_update_trx<'a>(
//     db: &'a mut impl Executor,
//     id: i32,
//     trx: UpdateTrx,
// ) -> BoxFuture<'a, MySqlQueryResult> {
//     async move {

//         let mut query = sqlx::QueryBuilder::new(r#"UPDATE tbltransactions SET "#);
//         let mut updates: Vec<UpdateQuery> = Vec::new();

//         if cat.name.is_some() {
//             updates.push(UpdateQuery {
//                 key: "name".to_string(),
//                 value: cat.name.unwrap().to_string(),
//             })
//         }

//         if cat.description.is_some() {
//             updates.push(UpdateQuery {
//                 key: "description".to_string(),
//                 value: cat.description.unwrap().to_string(),
//             })
//         }

//         let mut separated = query.separated(", ");
//         for update in updates.iter() {
//             separated.push(update.key.clone())
//                 .push_unseparated(" = ")
//                 .push_bind_unseparated(update.value.clone());
//         }

//         separated.push_unseparated(" WHERE id = ")
//             .push_bind_unseparated(id);
        
//         query.build()
//             .execute(db.as_executor())
//             .await
//             .unwrap();

//         let mut query = sqlx::QueryBuilder::new(r#"SELECT * FROM tbltransactions WHERE id = "#);
//         let trx = query
//             .push_bind(id)
//             .build_query_as::<ExistTrx>()
//             .fetch_one(db.as_executor())
//             .await
//             .unwrap();

//             trx
//     }
//     .boxed()
// }

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