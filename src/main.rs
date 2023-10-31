// #![deny(warnings)]

use crate::handlers::handler;

use bytes::Buf;
use futures_util::{stream, StreamExt};
use hyper::server::conn::AddrStream;
use hyper::client::HttpConnector;
use hyper::service::{make_service_fn, service_fn};
use hyper::{header, Body, Client, Method, Request, Response, Server, StatusCode};

type GenericError = Box<dyn std::error::Error + Send + Sync>;
type Result<T> = std::result::Result<T, GenericError>;

mod handlers;
mod models;
mod repositories;
// mod models;
// mod schemas;

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();

    let addr = ([127, 0, 0, 1], 1337).into();
    let client = Client::new();
    let fin_service = make_service_fn(|socket: &AddrStream| {

        let remote_addr = socket.remote_addr();

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