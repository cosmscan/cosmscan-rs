use hyper::{header, Body, Request, Response, StatusCode};

use crate::{AppState, GenericError};

pub async fn handle_hello_world(
    req: Request<Body>,
    state: AppState,
) -> Result<Response<Body>, GenericError> {
    let response = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from("{ \"message\": \"Hello World\" }"))?;
    Ok(response)
}
