use futures_util::{stream, StreamExt};
use hyper::{Body, Client};
use hyper::client::HttpConnector;
use hyper::http::{header, Method, Request, Response};

use crate::{POST_DATA, URL};

pub mod server;

pub async fn client_request_response(
    client: &Client<HttpConnector>
) -> crate::Result<Response<Body>> {
    let req = Request::builder()
        .method(Method::POST)
        .uri(URL)
        .header(header::CONTENT_TYPE, "application/json")
        .body(POST_DATA.into())
        .unwrap();

    let web_res = client.request(req).await?;

    let before = stream::once(async {
        Ok(
            format!(
                "<b>POST request body</b>: {}<br><b>Response</b>: ",
                POST_DATA,
            ).into()
        )
    });

    let after = web_res.into_body();
    let body = Body::wrap_stream(before.chain(after));

    Ok(Response::new(body))
}
