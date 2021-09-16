use hyper::http::{Request, Response, Method, StatusCode};
use hyper::{Body, Client};
use hyper::client::HttpConnector;
use futures_util::{stream, StreamExt};
use hyper::http::header::CONTENT_TYPE;
use crate::{NOT_FOUND, POST_DATA};

pub async fn match_api_v1(
    req: Request<Body>,
) -> crate::Result<Response<Body>> {
    match (req.method(), req.uri()) {
        (&Method::GET, _) => {
            Ok(Response::new(Body::from("GET api v1")))
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

pub async fn watcher_client(client: &Client<HttpConnector>) -> crate::Result<Response<Body>> {
    let req = Request::builder()
        .method(Method::GET)
        .uri("https://jsonplaceholder.typicode.com/posts/1")
        .header(CONTENT_TYPE, "application/json")
        .body(POST_DATA.into())
        .unwrap();

    let web_resp = client.request(req).await?;

    let before = stream::once(async {
        Ok(Response::new(Body::from("Ok")))
    });

    let after = web_resp.into_body();
    let body = Body::wrap_stream(before.chain(after));

    Ok(Response::new(body))
}