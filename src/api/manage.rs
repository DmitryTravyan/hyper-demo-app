use hyper::http::{Request, Response, Method, StatusCode};
use hyper::{Body, Client};
use hyper::client::HttpConnector;
use serde_json::json;
use crate::{HOST, PORT, NOT_FOUND};

pub async fn match_manage(
    req: Request<Body>,
    client: Client<HttpConnector>,
) -> crate::Result<Response<Body>> {
    let up = json!({
        "status": "up"
    });
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/manage/health") => {
            info!("GET http://{}:{}/manage/health from {:?}", HOST, PORT, req.headers());
            Ok(Response::new(Body::from(serde_json::to_vec(&up)?)))
        }
        _ => {
            Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(NOT_FOUND.into())
                .unwrap()
            )
        }
    }
}