// #![deny(warnings)]

use crate::handlers::handler;

use dotenv::dotenv;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Client, Server};

type GenericError = Box<dyn std::error::Error + Send + Sync>;
type Result<T> = std::result::Result<T, GenericError>;

mod handlers;
mod models;
mod repositories;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    pretty_env_logger::init();

    let addr = ([127, 0, 0, 1], 1337).into();
    let client = Client::new();
    let fin_service = make_service_fn(|_| {

        // Move a clone of `client` into the `service_fn`.
        let client = client.clone();
        async {
            Ok::<_, GenericError>(service_fn(move |req| {
                // Clone again to ensure that client outlives this closure.
                handler(req, client.to_owned())
            }))
        }

    });

    let server = Server::bind(&addr).serve(fin_service);
    println!("Listening on http://{}", addr);
    server.await?;

    Ok(())
}