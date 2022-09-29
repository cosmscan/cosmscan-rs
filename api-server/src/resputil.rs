use hyper::{header, Body, Response, StatusCode};

use crate::GenericError;

pub struct ResponseBuilder {
    pub allowed_host: String,
}

impl ResponseBuilder {
    pub fn new(allowed_host: String) -> Self {
        Self { allowed_host }
    }

    pub fn invalid_form(&self, error: &str) -> Result<Response<Body>, GenericError> {
        let response = Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .header(header::CONTENT_TYPE, "application/json")
            .header(
                header::ACCESS_CONTROL_ALLOW_ORIGIN,
                self.allowed_host.clone(),
            )
            .body(Body::from(format!("{{ \"error\": \"{}\" }}", error)))?;
        Ok(response)
    }

    pub fn ok_json(&self, json: String) -> Result<Response<Body>, GenericError> {
        let response = Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "application/json")
            .header(
                header::ACCESS_CONTROL_ALLOW_ORIGIN,
                self.allowed_host.clone(),
            )
            .body(Body::from(json))?;
        Ok(response)
    }

    pub fn not_found(&self) -> Result<Response<Body>, GenericError> {
        let response = Response::builder()
            .status(StatusCode::NOT_FOUND)
            .header(header::CONTENT_TYPE, "application/json")
            .header(
                header::ACCESS_CONTROL_ALLOW_ORIGIN,
                self.allowed_host.clone(),
            )
            .body(Body::from("{ \"error\": \"content not found\"}"))?;
        Ok(response)
    }

    pub fn internal_error(&self) -> Result<Response<Body>, GenericError> {
        let response = Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .header(header::CONTENT_TYPE, "application/json")
            .header(
                header::ACCESS_CONTROL_ALLOW_ORIGIN,
                self.allowed_host.clone(),
            )
            .body(Body::from("{ \"error\": \"Internal Server Error\" }"))?;
        Ok(response)
    }
}
