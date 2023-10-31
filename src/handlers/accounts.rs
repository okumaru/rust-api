
use crate::handlers::req_query_id;
use crate::models::accounts::{ new_account, AccountModel, ExistAccount, NewAccount };
use crate::repositories::accounts::{AccountRepo, AccountTrait};

use bytes::Buf;
use sqlx::mysql::MySqlPool;
use hyper::body::{Body as BBody, to_bytes, HttpBody};
use hyper::{header, Client, Body, Method, Request, Response, StatusCode};

type GenericError = Box<dyn std::error::Error + Send + Sync>;
type Result<T> = std::result::Result<T, GenericError>;

static INTERNAL_SERVER_ERROR: &[u8] = b"Internal Server Error";
static NOTFOUND: &[u8] = b"Not Found";

pub struct AccountHandler<'a>{
    account_repo: AccountRepo,
    request: &'a Request<Body>,
}

impl<'a> AccountHandler<'a> {
    pub fn new(req: &'a Request<Body>, pool: MySqlPool) -> Self {
        Self { 
            account_repo: AccountRepo::new(pool),
            request: req,
        }
    }

    async fn list(&mut self) -> Result<Response<Body>> {

        let datas = self.account_repo.account_list().await?;

        let accounts: Vec<AccountModel> = datas.iter().map(|account| new_account(
            &account.id, 
            &account.name, 
            &account.description, 
            &account.balance,
            &account.created_at,
            &account.updated_at,
        )).collect();

        let res = match serde_json::to_string(&accounts) {
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
        let datas = self.account_repo.account_detail(query_id).await?;

        let account = new_account(
            &datas.id, 
            &datas.name, 
            &datas.description, 
            &datas.balance,
            &datas.created_at,
            &datas.updated_at,
        );

        let res = match serde_json::to_string(&account) {
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

        let data: NewAccount = serde_json::from_str(body)?;
        let new_acc = self.account_repo.account_add(data.clone()).await?;

        let account = new_account(
            &new_acc.id, 
            &new_acc.name, 
            &new_acc.description, 
            &new_acc.balance,
            &new_acc.created_at,
            &new_acc.updated_at,
        );

        // let data = vec!["foo", "bar"];
        let res = match serde_json::to_string(&account) {
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

    let pool = MySqlPool::connect("mysql://root:local@localhost:3306/sohfin").await?;

    let (mut parts, body) = req.into_parts();
    let body_bytes = hyper::body::to_bytes(body).await.unwrap();
    let body = std::str::from_utf8(&body_bytes).unwrap();
    
    let request: hyper::Request<Body> = Request::from_parts(parts, body_bytes.clone().into());
    let mut account_handler = AccountHandler::new(&request, pool);

    match (request.method(), request.uri().query().is_none()) {

        (&Method::GET, true) => account_handler.list().await,
        (&Method::GET, false) => account_handler.detail().await,
        (&Method::PUT, true) => account_handler.add(body).await,
        // (&Method::POST, false) => api_post_accounts(&pool, query_id, req).await,
        // &Method::DELETE => api_del_accounts(req).await,

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