
use crate::handlers::req_query_id;
use crate::models::bigdecimal_to_int;
use crate::models::trx_cats::{ TrxCatModel, ExistTrxCatWithBudget, AddTrxCat, UpdateTrxCat, build_model_from_exist };
use crate::models::trx_cat_budgets::{TrxCatBudgetModel};
use crate::repositories::trx_cats::{TrxCatRepo, TrxCatTrait};

use std::env;
use sqlx::mysql::MySqlPool;
use hyper::{header, Body, Method, Request, Response, StatusCode};

type GenericError = Box<dyn std::error::Error + Send + Sync>;
type Result<T> = std::result::Result<T, GenericError>;

static INTERNAL_SERVER_ERROR: &[u8] = b"Internal Server Error";
static NOTFOUND: &[u8] = b"Not Found";

pub struct TrxCatHandler<'a>{
    trx_cat_repo: TrxCatRepo,
    request: &'a Request<Body>,
}

impl<'a> TrxCatHandler<'a> {
    pub fn new(req: &'a Request<Body>, pool: MySqlPool) -> Self {
        Self { 
            trx_cat_repo: TrxCatRepo::new(pool),
            request: req,
        }
    }

    async fn list(&mut self) -> Result<Response<Body>> {

        let datas: Vec<ExistTrxCatWithBudget> = self.trx_cat_repo.trx_cats_list().await?;
        let cats: Vec<TrxCatModel> = datas.iter().map(|data| build_model_from_exist(data.clone())).collect();

        let res = match serde_json::to_string(&cats) {
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
        let data: ExistTrxCatWithBudget = self.trx_cat_repo.trx_cats_detail(query_id).await?;
        let cat: TrxCatModel = build_model_from_exist(data);

        let res = match serde_json::to_string(&cat) {
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

        let data: AddTrxCat = serde_json::from_str(body)?;
        let new_cat: ExistTrxCatWithBudget = self.trx_cat_repo.trx_cats_add(data.clone()).await?;
        let cat: TrxCatModel = build_model_from_exist(new_cat);

        let res = match serde_json::to_string(&cat) {
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
        let data: UpdateTrxCat = serde_json::from_str(body)?;
        let update_cat = self.trx_cat_repo.trx_cats_update(query_id, data.clone()).await?;
        let cat: TrxCatModel = build_model_from_exist(update_cat);

        let res = match serde_json::to_string(&cat) {
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
        let delete_cat = self.trx_cat_repo.trx_cats_delete(query_id).await?;
        let cat: TrxCatModel = build_model_from_exist(delete_cat);

        let res = match serde_json::to_string(&cat) {
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
    let mut trx_cat_handler = TrxCatHandler::new(&request, pool);

    match (request.method(), request.uri().query().is_none()) {

        (&Method::GET, true) => trx_cat_handler.list().await,
        (&Method::GET, false) => trx_cat_handler.detail().await,
        (&Method::PUT, true) => trx_cat_handler.add(body).await,
        (&Method::POST, false) => trx_cat_handler.update(body).await,
        (&Method::DELETE, false) => trx_cat_handler.delete().await,

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