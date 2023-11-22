
use crate::handlers::accounts as accounts_handlers;
use crate::handlers::cat_types as cat_types_handlers;
use crate::handlers::trx_cats as trx_cats_handlers;
use crate::handlers::trx_cat_budgets as trx_cat_budgets_handlers;
use crate::handlers::trxs as trxs_handlers;

use std::collections::HashMap;
use hyper::client::HttpConnector;
use hyper::{Body, Client, Request, Response, StatusCode};

type GenericError = Box<dyn std::error::Error + Send + Sync>;
type Result<T> = std::result::Result<T, GenericError>;

pub mod accounts;
pub mod cat_types;
pub mod trx_cats;
pub mod trx_cat_budgets;
pub mod trxs;

static INDEX: &[u8] = b"test";
static NOTFOUND: &[u8] = b"Not Found";

pub async fn handler(
    req: Request<Body>,
    _client: Client<HttpConnector>,
) -> Result<Response<Body>> {

    let default_pages = vec!["/", "/index.html"];

    if default_pages.contains(&req.uri().path()) {
        return Ok(Response::new(INDEX.into()))
    }

    match req.uri().path() {
        "/accounts" => accounts_handlers::handler(req).await,
        "/cat_types" => cat_types_handlers::handler(req).await,
        "/trx_cats" => trx_cats_handlers::handler(req).await,
        "/trx_cats_budgets" => trx_cat_budgets_handlers::handler(req).await,
        "/trxs" => trxs_handlers::handler(req).await,
        _ => {
            // Return 404 not found response.
            Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(NOTFOUND.into())
                .unwrap())
        }
    }
}

pub fn req_query_id( req: &Request<Body> ) -> i32 {

    let queries = req.uri().query();
    if queries.is_none() {
        return 0;
    }

    // get data with filter query
    let params: HashMap<String, String> = req
        .uri()
        .query()
        .map(|v| {
            url::form_urlencoded::parse(v.as_bytes())
                .into_owned()
                .collect()
        })
        .unwrap_or_else(HashMap::new);

    let str_account_id = params.get("id").unwrap().to_string();
    return str_account_id.parse::<i32>().unwrap();

}