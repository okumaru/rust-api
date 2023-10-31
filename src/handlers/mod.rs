
use crate::handlers::accounts as accounts_handlers;

use std::collections::HashMap;
use bytes::Buf;
use futures_util::{stream, StreamExt};
use sqlx::mysql::MySqlPool;
use hyper::server::conn::AddrStream;
use hyper::client::HttpConnector;
use hyper::service::{make_service_fn, service_fn};
use hyper::{header, Body, Client, Method, Request, Response, Server, StatusCode};

type GenericError = Box<dyn std::error::Error + Send + Sync>;
type Result<T> = std::result::Result<T, GenericError>;

pub mod accounts;
// pub mod trxs;
// pub mod trx_cats;

static INDEX: &[u8] = b"test";
static NOTFOUND: &[u8] = b"Not Found";

pub async fn handler(
    req: Request<Body>,
    client: Client<HttpConnector>,
) -> Result<Response<Body>> {

    let default_pages = vec!["/", "/index.html"];

    if default_pages.contains(&req.uri().path()) {
        return Ok(Response::new(INDEX.into()))
    }

    match req.uri().path() {
        "/accounts" => accounts_handlers::handler(req).await,
        // "/trxs" => api_post_response(req).await,
        // "/trx_cats" => api_get_response().await,
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