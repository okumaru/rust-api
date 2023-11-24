
use crate::handlers::req_query_id;
use crate::models::bigdecimal_to_int;
use crate::models::trxs::{ TrxModel, TrxModelWithAccCat, NewTrx, UpdateTrx, build_model_from_exist };
use crate::repositories::trxs::{TrxRepo, TrxTrait};

use std::env;
use sqlx::mysql::MySqlPool;
use hyper::{header, Body, Method, Request, Response, StatusCode};

type GenericError = Box<dyn std::error::Error + Send + Sync>;
type Result<T> = std::result::Result<T, GenericError>;

static INTERNAL_SERVER_ERROR: &[u8] = b"Internal Server Error";
static NOTFOUND: &[u8] = b"Not Found";

pub struct TrxHandler<'a>{
    trx_repo: TrxRepo,
    request: &'a Request<Body>,
}

impl<'a> TrxHandler<'a> {
    pub fn new(req: &'a Request<Body>, pool: MySqlPool) -> Self {
        Self { 
            trx_repo: TrxRepo::new(pool),
            request: req,
        }
    }

    async fn list(&mut self) -> Result<Response<Body>> {

        let datas = self.trx_repo.trxs_list().await?;
        let trxs: Vec<TrxModelWithAccCat> = datas.iter().map(|data| build_model_from_exist(data.clone())).collect();

        let res = match serde_json::to_string(&trxs) {
            Ok(json) => Response::builder()
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(json))
                .unwrap(),
            Err(_) => Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(INTERNAL_SERVER_ERROR.into())
                .unwrap(),
        };
        Ok(res)
    }

    async fn detail(&mut self) -> Result<Response<Body>> { 

        let query_id = req_query_id(self.request);
        let data = self.trx_repo.trx_detail(query_id).await?;
        let trx = build_model_from_exist(data);

        let res = match serde_json::to_string(&trx) {
            Ok(json) => Response::builder()
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(json))
                .unwrap(),
            Err(_) => Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(INTERNAL_SERVER_ERROR.into())
                .unwrap(),
        };
        Ok(res)
    }

    async fn add(&mut self, body: &str) -> Result<Response<Body>> { 

        let data: NewTrx = serde_json::from_str(body)?;
        let new_trx = self.trx_repo.trx_add(data.clone()).await?;

        let trx = TrxModel {
            id: new_trx.id,
            credit: bigdecimal_to_int(new_trx.credit),
            debit: bigdecimal_to_int(new_trx.debit),
            description: new_trx.description,
            balance_before: bigdecimal_to_int(new_trx.balance_before),
            balance_after: bigdecimal_to_int(new_trx.balance_after),
            datetime: new_trx.datetime,
            created_at: new_trx.created_at,
            updated_at: new_trx.updated_at,
            accountid: new_trx.accountid,
            categoryid: new_trx.categoryid,
        };

        let res = match serde_json::to_string(&trx) {
            Ok(json) => Response::builder()
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(json))
                .unwrap(),
            Err(_) => Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(INTERNAL_SERVER_ERROR.into())
                .unwrap(),
        };
        Ok(res)
    }

    async fn update(&mut self, body: &str) -> Result<Response<Body>> { 

        let query_id = req_query_id(self.request);
        let data: UpdateTrx = serde_json::from_str(body)?;
        let update_trx = self.trx_repo.trx_update(query_id, data.clone()).await?;

        let trx = TrxModel {
            id: update_trx.id,
            credit: bigdecimal_to_int(update_trx.credit),
            debit: bigdecimal_to_int(update_trx.debit),
            description: update_trx.description,
            balance_before: bigdecimal_to_int(update_trx.balance_before),
            balance_after: bigdecimal_to_int(update_trx.balance_after),
            datetime: update_trx.datetime,
            created_at: update_trx.created_at,
            updated_at: update_trx.updated_at,
            accountid: update_trx.accountid,
            categoryid: update_trx.categoryid,
        };

        let res = match serde_json::to_string(&trx) {
            Ok(json) => Response::builder()
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(json))
                .unwrap(),
            Err(_) => Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(INTERNAL_SERVER_ERROR.into())
                .unwrap(),
        };
        Ok(res)
    }

    async fn delete(&mut self) -> Result<Response<Body>> { 

        let query_id = req_query_id(self.request);
        let delete_trx = self.trx_repo.trx_delete(query_id).await?;

        let trx = TrxModel {
            id: delete_trx.id,
            credit: bigdecimal_to_int(delete_trx.credit),
            debit: bigdecimal_to_int(delete_trx.debit),
            description: delete_trx.description,
            balance_before: bigdecimal_to_int(delete_trx.balance_before),
            balance_after: bigdecimal_to_int(delete_trx.balance_after),
            datetime: delete_trx.datetime,
            created_at: delete_trx.created_at,
            updated_at: delete_trx.updated_at,
            accountid: delete_trx.accountid,
            categoryid: delete_trx.categoryid,
        };

        let res = match serde_json::to_string(&trx) {
            Ok(json) => Response::builder()
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(json))
                .unwrap(),
            Err(_) => Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(INTERNAL_SERVER_ERROR.into())
                .unwrap(),
        };
        Ok(res)
    }
}

pub async fn handler( req: Request<Body> ) -> Result<Response<Body>> {
    let pool = MySqlPool::connect(&env::var("DATABASE_URL").unwrap()).await?;

    let (parts, body) = req.into_parts();
    let body_bytes = hyper::body::to_bytes(body).await.unwrap();
    let body = std::str::from_utf8(&body_bytes).unwrap();
    
    let request: hyper::Request<Body> = Request::from_parts(parts, body_bytes.clone().into());
    let mut trx_handler = TrxHandler::new(&request, pool);

    match (request.method(), request.uri().query().is_none()) {

        (&Method::GET, true) => trx_handler.list().await,
        (&Method::GET, false) => trx_handler.detail().await,
        (&Method::PUT, true) => trx_handler.add(body).await,
        (&Method::POST, false) => trx_handler.update(body).await,
        (&Method::DELETE, false) => trx_handler.delete().await,

        // 
        _ => {
            // Return 404 not found response.
            Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(NOTFOUND.into())
                .unwrap())
        }
        
    }

}