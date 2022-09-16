use cosmscan_models::storage::StorageReader;
use hyper::{header, Body, Request, Response, StatusCode};

use crate::{resputil, AppState, GenericError};

/// Returns a block by height.
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

/// Returns a latestblock
pub async fn latest_block(
    req: Request<Body>,
    state: AppState,
) -> Result<Response<Body>, GenericError> {
    let storage = state.storage;
    let block = storage.find_latest_block()?;
    let json = serde_json::to_string(&block)?;

    resputil::ok_json(json)
}
