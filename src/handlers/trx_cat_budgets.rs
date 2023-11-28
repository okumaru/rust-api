
use crate::handlers::{req_query_id, get_req_query};
use crate::models::bigdecimal_to_int;
use crate::models::trx_cat_budgets::{ TrxCatBudgetModel, NewTrxCatBudget, UpdateTrxCatBudget };
use crate::repositories::trx_cat_budgets::{TrxCatBudgetRepo, TrxCatBudgetTrait};

use std::env;
use sqlx::mysql::MySqlPool;
use hyper::{header, Body, Method, Request, Response, StatusCode};

type GenericError = Box<dyn std::error::Error + Send + Sync>;
type Result<T> = std::result::Result<T, GenericError>;

static INTERNAL_SERVER_ERROR: &[u8] = b"Internal Server Error";
static NOTFOUND: &[u8] = b"Not Found";

pub struct TrxCatBudgetHandler<'a>{
    trx_cat_budget_repo: TrxCatBudgetRepo,
    request: &'a Request<Body>,
}

impl<'a> TrxCatBudgetHandler<'a> {
    pub fn new(req: &'a Request<Body>, pool: MySqlPool) -> Self {
        Self { 
            trx_cat_budget_repo: TrxCatBudgetRepo::new(pool),
            request: req,
        }
    }

    async fn list(&mut self) -> Result<Response<Body>> { 

        let str_category_id: Option<String> = get_req_query(self.request, String::from("categoryid"));
        let int_category_id: i32 = str_category_id.unwrap_or("0".to_string()).parse().ok().unwrap_or_default();
        let datas = self.trx_cat_budget_repo.trx_cat_budget_list(int_category_id).await?;

        let budget: Vec<TrxCatBudgetModel> = datas.iter().map(|data| TrxCatBudgetModel {
            id: data.id,
            periode: data.periode.clone(),
            allocated: bigdecimal_to_int(data.allocated.clone()),
            spent: bigdecimal_to_int(data.spent.clone()),
            available: bigdecimal_to_int(data.available.clone()),
            created_at: data.created_at,
            updated_at: data.updated_at,
            categoryid: data.categoryid,
            }
        ).collect();

        let res = match serde_json::to_string(&budget) {
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
        let data = self.trx_cat_budget_repo.trx_cat_budget_detail(query_id).await?;

        let budget = TrxCatBudgetModel {
            id: data.id,
            periode: data.periode,
            allocated: bigdecimal_to_int(data.allocated),
            spent: bigdecimal_to_int(data.spent),
            available: bigdecimal_to_int(data.available),
            created_at: data.created_at,
            updated_at: data.updated_at,
            categoryid: data.categoryid,
        };

        let res = match serde_json::to_string(&budget) {
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

        let data: NewTrxCatBudget = serde_json::from_str(body)?;
        let new_budget = self.trx_cat_budget_repo.trx_cat_budget_add(data.clone()).await?;

        let budget = TrxCatBudgetModel {
            id: new_budget.id,
            periode: new_budget.periode,
            allocated: bigdecimal_to_int(new_budget.allocated),
            spent: bigdecimal_to_int(new_budget.spent),
            available: bigdecimal_to_int(new_budget.available),
            created_at: new_budget.created_at,
            updated_at: new_budget.updated_at,
            categoryid: new_budget.categoryid,
        };

        let res = match serde_json::to_string(&budget) {
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
        let data: UpdateTrxCatBudget = serde_json::from_str(body)?;
        let update_budget = self.trx_cat_budget_repo.trx_cat_budget_update(query_id, data.clone()).await?;

        let budget = TrxCatBudgetModel {
            id: update_budget.id,
            periode: update_budget.periode,
            allocated: bigdecimal_to_int(update_budget.allocated),
            spent: bigdecimal_to_int(update_budget.spent),
            available: bigdecimal_to_int(update_budget.available),
            created_at: update_budget.created_at,
            updated_at: update_budget.updated_at,
            categoryid: update_budget.categoryid,
        };

        let res = match serde_json::to_string(&budget) {
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
        let delete_budget = self.trx_cat_budget_repo.trx_cat_budget_delete(query_id).await?;

        let budget = TrxCatBudgetModel {
            id: delete_budget.id,
            periode: delete_budget.periode,
            allocated: bigdecimal_to_int(delete_budget.allocated),
            spent: bigdecimal_to_int(delete_budget.spent),
            available: bigdecimal_to_int(delete_budget.available),
            created_at: delete_budget.created_at,
            updated_at: delete_budget.updated_at,
            categoryid: delete_budget.categoryid,
        };

        let res = match serde_json::to_string(&budget) {
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
    let mut trx_cat_budget_handler = TrxCatBudgetHandler::new(&request, pool);
    let is_specified: bool = match get_req_query(&request, "id".to_string()) {
        Some(_) => true,
        None => false
    };

    match (request.method(), is_specified) {

        (&Method::GET, false) => trx_cat_budget_handler.list().await,
        (&Method::GET, true) => trx_cat_budget_handler.detail().await,
        (&Method::PUT, false) => trx_cat_budget_handler.add(body).await,
        (&Method::POST, true) => trx_cat_budget_handler.update(body).await,
        (&Method::DELETE, true) => trx_cat_budget_handler.delete().await,

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