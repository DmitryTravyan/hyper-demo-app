use hyper::http::{Response, Request, StatusCode, header};
use hyper::Body;
use bytes::Buf;

pub async fn api_get_response(req: Request<Body>) -> crate::Result<Response<Body>> {
    let whole_body = hyper::body::aggregate(req).await?;

    let mut data: serde_json::Value = serde_json::from_reader(whole_body.reader())?;

    data["test"] = serde_json::Value::from("test_value");

    let json = serde_json::to_string(&data)?;
    let response = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(json))?;

    Ok(response)
}