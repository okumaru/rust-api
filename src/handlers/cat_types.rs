
use crate::handlers::req_query_id;
use crate::models::bigdecimal_to_int;
use crate::models::cat_types::{ CatTypeModel, AddCatType, UpdateCatType };
use crate::repositories::cat_types::{CatTypeRepo, CatTypeTrait};

use std::env;
use sqlx::mysql::MySqlPool;
use hyper::{header, Body, Method, Request, Response, StatusCode};

type GenericError = Box<dyn std::error::Error + Send + Sync>;
type Result<T> = std::result::Result<T, GenericError>;

static INTERNAL_SERVER_ERROR: &[u8] = b"Internal Server Error";
static NOTFOUND: &[u8] = b"Not Found";

pub struct CatTypeHandler<'a>{
    cat_type_repo: CatTypeRepo,
    request: &'a Request<Body>,
}

impl<'a> CatTypeHandler<'a> {
    pub fn new(req: &'a Request<Body>, pool: MySqlPool) -> Self {
        Self { 
            cat_type_repo: CatTypeRepo::new(pool),
            request: req,
        }
    }

    async fn list(&mut self) -> Result<Response<Body>> {

        let datas = self.cat_type_repo.cat_types_list().await?;
        let types: Vec<CatTypeModel> = datas.iter().map(|data| CatTypeModel {
            id: data.id, 
            r#type: data.r#type.clone(), 
            description: data.description.clone(),  
            target: bigdecimal_to_int(data.target.clone()),  
            available: bigdecimal_to_int(data.available.clone()),  
            icon: data.icon.clone(),  
            created_at: data.created_at,
            updated_at: data.updated_at,
        }).collect();

        let res = match serde_json::to_string(&types) {
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
        let data = self.cat_type_repo.cat_type_detail(query_id).await?;

        let cat_type = CatTypeModel {
            id: data.id, 
            r#type: data.r#type.clone(), 
            description: data.description.clone(),  
            target: bigdecimal_to_int(data.target.clone()),  
            available: bigdecimal_to_int(data.available.clone()),  
            icon: data.icon.clone(),  
            created_at: data.created_at,
            updated_at: data.updated_at,
        };

        let res = match serde_json::to_string(&cat_type) {
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

        let data: AddCatType = serde_json::from_str(body)?;
        let new_type = self.cat_type_repo.cat_type_add(data.clone()).await?;

        let cat_type = CatTypeModel {
            id: new_type.id, 
            r#type: new_type.r#type.clone(), 
            description: new_type.description.clone(),  
            target: bigdecimal_to_int(new_type.target.clone()),  
            available: bigdecimal_to_int(new_type.available.clone()),  
            icon: new_type.icon.clone(),  
            created_at: new_type.created_at,
            updated_at: new_type.updated_at,
        };

        let res = match serde_json::to_string(&cat_type) {
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
        let data: UpdateCatType = serde_json::from_str(body)?;
        let update_type = self.cat_type_repo.cat_type_update(query_id, data.clone()).await?;

        let cat_type = CatTypeModel {
            id: update_type.id, 
            r#type: update_type.r#type.clone(), 
            description: update_type.description.clone(),  
            target: bigdecimal_to_int(update_type.target.clone()),  
            available: bigdecimal_to_int(update_type.available.clone()),  
            icon: update_type.icon.clone(),  
            created_at: update_type.created_at,
            updated_at: update_type.updated_at,
        };

        let res = match serde_json::to_string(&cat_type) {
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
        let delete_cat = self.cat_type_repo.cat_type_delete(query_id).await?;

        let cat_type = CatTypeModel {
            id: delete_cat.id, 
            r#type: delete_cat.r#type.clone(), 
            description: delete_cat.description.clone(),  
            target: bigdecimal_to_int(delete_cat.target.clone()),  
            available: bigdecimal_to_int(delete_cat.available.clone()),  
            icon: delete_cat.icon.clone(),  
            created_at: delete_cat.created_at,
            updated_at: delete_cat.updated_at,
        };

        let res = match serde_json::to_string(&cat_type) {
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
    let mut cat_type_handler = CatTypeHandler::new(&request, pool);

    match (request.method(), request.uri().query().is_none()) {

        (&Method::GET, true) => cat_type_handler.list().await,
        (&Method::GET, false) => cat_type_handler.detail().await,
        (&Method::PUT, true) => cat_type_handler.add(body).await,
        (&Method::POST, false) => cat_type_handler.update(body).await,
        (&Method::DELETE, false) => cat_type_handler.delete().await,

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