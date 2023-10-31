
use crate::handlers::accounts;
// use crate::handlers::trxs::trxs_handler;
// use crate::handlers::trx_cats::trx_cats_handler;

use hyper::{Body, Request, Method, Response, StatusCode};

type GenericError = Box<dyn std::error::Error + Send + Sync>;
type Result<T> = std::result::Result<T, GenericError>;

static INDEX: &[u8] = b"test";
static NOTFOUND: &[u8] = b"Not Found";

pub async fn handler(
    req: Request<Body>,
) -> Result<Response<Body>> {

    let default_uris = vec!["/", "/index.html"];

    if default_uris.iter().any(|&uri| uri == req.uri().path()) {
        return Ok(Response::new(INDEX.into()))
    }
    
    match req.uri().path() {
        "/accounts" => accounts::handler(req).await,
        // "/trxs" => trxs_handler(req).await,
        // "/trx-cats" => trx_cats_handler(req).await,
        _ => {
            Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(NOTFOUND.into())
                .unwrap())
        }
    }
}