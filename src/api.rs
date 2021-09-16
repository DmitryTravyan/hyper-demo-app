mod v1;
mod v2;
mod manage;

use hyper::http::{Request, Response, StatusCode, Method};
use hyper::client::HttpConnector;
use hyper::{Body, Client};

use crate::{Result, NOT_FOUND, INDEX, HOST, PORT};
use crate::client::client_request_response;
use crate::post::api_get_response;
use crate::api::manage::match_manage;
use crate::api::v1::match_api_v1;
use crate::api::v2::match_api_v2;

pub async fn match_response(
    req: Request<Body>,
    client: Client<HttpConnector>,
) -> Result<Response<Body>> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, p) if p.starts_with("/manage") => {
            match_manage(req, client).await
        }
        (_, path) if path.starts_with("/api/v1") => {
            match_api_v1(req).await
        },
        (_, path) if path.starts_with("/api/v2") => {
            match_api_v2(req, client).await
        },
        (&Method::GET, "/") | (&Method::GET, "/index.html") => {
            Ok(Response::new(INDEX.into()))
        }
        (&Method::POST, "/test.html") => {
            client_request_response(&client).await
        },
        _ => {
            Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(NOT_FOUND.into())
                .unwrap()
            )
        }
    }
}

