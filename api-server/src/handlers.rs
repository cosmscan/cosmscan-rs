use cosmscan_models::storage::StorageReader;
use hyper::{header, Body, Request, Response, StatusCode};

use crate::{resputil, AppState, GenericError};

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

pub async fn get_block(
    req: Request<Body>,
    state: AppState,
) -> Result<Response<Body>, GenericError> {
    let block_height = match state.params.find("block_height") {
        Some(block_height) => block_height.parse::<i64>()?,
        None => {
            return resputil::invalid_form("block_height is missing");
        }
    };

    let storage = state.storage;
    let block = storage.find_block_by_height(block_height)?;
    let json = serde_json::to_string(&block)?;

    resputil::ok_json(json)
}
