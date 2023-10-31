
use std::env;
use sqlx::mysql::MySqlPool;
use hyper::{header, Body, Method, Request, Response, StatusCode};

type GenericError = Box<dyn std::error::Error + Send + Sync>;
type Result<T> = std::result::Result<T, GenericError>;

static INTERNAL_SERVER_ERROR: &[u8] = b"Internal Server Error";
static NOTFOUND: &[u8] = b"Not Found";

async fn api_get_trxs(
    _pool: &MySqlPool,
) -> Result<Response<Body>> {
    let data = vec!["foo", "bar"];
    let res = match serde_json::to_string(&data) {
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

// async fn api_put_trxs(
//     req: Request<Body>,
// ) -> Result<Response<Body>> {
//     let data = vec!["foo", "bar"];
//     let res = match serde_json::to_string(&data) {
//         Ok(json) => Response::builder()
//             .header(header::CONTENT_TYPE, "application/json")
//             .body(Body::from(json))
//             .unwrap(),
//         Err(_) => Response::builder()
//             .status(StatusCode::INTERNAL_SERVER_ERROR)
//             .body(INTERNAL_SERVER_ERROR.into())
//             .unwrap(),
//     };
//     Ok(res)
// }

// async fn api_post_trxs(
//     req: Request<Body>,
// ) -> Result<Response<Body>> {
//     let data = vec!["foo", "bar"];
//     let res = match serde_json::to_string(&data) {
//         Ok(json) => Response::builder()
//             .header(header::CONTENT_TYPE, "application/json")
//             .body(Body::from(json))
//             .unwrap(),
//         Err(_) => Response::builder()
//             .status(StatusCode::INTERNAL_SERVER_ERROR)
//             .body(INTERNAL_SERVER_ERROR.into())
//             .unwrap(),
//     };
//     Ok(res)
// }

// async fn api_delete_trxs(
//     req: Request<Body>,
// ) -> Result<Response<Body>> {
//     let data = vec!["foo", "bar"];
//     let res = match serde_json::to_string(&data) {
//         Ok(json) => Response::builder()
//             .header(header::CONTENT_TYPE, "application/json")
//             .body(Body::from(json))
//             .unwrap(),
//         Err(_) => Response::builder()
//             .status(StatusCode::INTERNAL_SERVER_ERROR)
//             .body(INTERNAL_SERVER_ERROR.into())
//             .unwrap(),
//     };
//     Ok(res)
// }

pub async fn trx_cats_handler(
    // pool: &MySqlPool,
    req: Request<Body>,
    // client: Client<HttpConnector>,
) -> Result<Response<Body>> {

    let pool = MySqlPool::connect(&env::var("DATABASE_URL")?).await?;

    match req.method() {
        &Method::GET => api_get_trxs(&pool).await,
        // &Method::PUT => api_put_trxs(req).await,
        // &Method::POST => api_post_trxs(req).await,
        // &Method::DELETE => api_delete_trxs(req).await,

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