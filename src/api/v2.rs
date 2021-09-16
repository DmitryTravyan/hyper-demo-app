use hyper::http::{Request, Response, Method, StatusCode};
use hyper::{Body, Client};
use hyper::client::HttpConnector;
use crate::NOT_FOUND;

pub async fn match_api_v2(
    req: Request<Body>,
    client: Client<HttpConnector>
) -> crate::Result<Response<Body>> {
    match (req.method(), req.uri()) {
        (&Method::GET, p) => {
            Ok(Response::new(Body::from("GET api v2")))
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