#[macro_use]
extern crate log;

use env_logger::Env;
#[deny(unused)]
use hyper::{Client, Server};
use hyper::client::conn::Connection;
use hyper::service::{make_service_fn, service_fn};
#[allow(unused)]
use log::{debug, error, info, trace, warn};

use client::server::shutdown_signal;

use crate::api::match_response;

mod client;
mod api;
mod post;
mod get;
mod domain;

static INDEX: &[u8] = b"<a href=\"test.html\">test.html</a>";
static INTERNAL_SERVER_ERROR: &[u8] = b"Internal Server Error";
static NOT_FOUND: &[u8] = b"Not Found";
static POST_DATA: &str = r#"{"original": "data"}"#;
static URL: &str = "http://127.0.0.1:1337/json_api";
static HOST: &str = "0.0.0.0";
static PORT: &str = "8090";

type GenericError = Box<dyn std::error::Error + Send + Sync>;
type Result<T> = std::result::Result<T, GenericError>;

#[tokio::main]
async fn main() -> crate::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    info!("Server starting at http://{}:{}", HOST, PORT);

    let addr = format!("{}:{}", HOST, PORT).parse().unwrap();
    let client = Client::new();

    let main_service = make_service_fn(move |_| {
        let client = client.clone();

        async {
            Ok::<_, GenericError>(service_fn(move |req| {
                match_response(req, client.to_owned())
            }))
        }
    });

    let server = Server::bind(&addr)
        .serve(main_service);
    let graceful =  server.with_graceful_shutdown(
        shutdown_signal()
    );
    if let Err(e) = graceful.await {
        warn!("server error: {}", e);
    }
    Ok(())
}
