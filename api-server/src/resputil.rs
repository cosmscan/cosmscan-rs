use hyper::{header, Body, Response, StatusCode};

use crate::GenericError;

pub fn invalid_form(error: &str) -> Result<Response<Body>, GenericError> {
    let response = Response::builder()
        .status(StatusCode::BAD_REQUEST)
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(format!("{{ \"error\": \"{}\" }}", error)))?;
    Ok(response)
}

pub fn ok_json(json: String) -> Result<Response<Body>, GenericError> {
    let response = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(json))?;
    Ok(response)
}

pub fn not_found() -> Result<Response<Body>, GenericError> {
    let response = Response::builder()
        .status(StatusCode::NOT_FOUND)
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from("{ \"error\": \"content not found\"}"))?;
    Ok(response)
}
