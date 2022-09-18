use std::collections::HashMap;

use cosmscan_models::storage::StorageReader;
use hyper::{header, Body, Request, Response, StatusCode};
use url::Url;

use crate::{resputil, AppState, GenericError};

pub async fn all_chains(
    req: Request<Body>,
    state: AppState,
) -> Result<Response<Body>, GenericError> {
    let storage = state.storage;
    let chains = storage.all_chains()?;
    let json = serde_json::to_string(&chains)?;

    resputil::ok_json(json)
}

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
    _: Request<Body>,
    state: AppState,
) -> Result<Response<Body>, GenericError> {
    let storage = state.storage;
    let block = storage.find_latest_block()?;
    let json = serde_json::to_string(&block)?;

    resputil::ok_json(json)
}

/// Returns list of blocks by given chain_id
/// it's sorted by height in descending order
pub async fn block_list(
    req: Request<Body>,
    state: AppState
) -> Result<Response<Body>, GenericError> {
    // parse limit and offset from query string
    let uri = req.uri().to_string();
    let query_pairs:HashMap<_,_> = Url::parse(&uri)?.query_pairs().into_owned().collect();
    let limit = match query_pairs.get("limit") {
        Some(limit) => limit.parse::<i64>()?,
        None => 10,
    };
    
    let offset = match query_pairs.get("offset") {
        Some(offset) => offset.parse::<i64>()?,
        None => 0,
    };
    
    let chain_id = match state.params.find("chain_id") {
        Some(chain_id) => chain_id.parse::<i32>()?,
        None => {
            return resputil::invalid_form("chain_id is missing");
        }
    };

    let storage = state.storage;
    let blocks = storage.list_blocks(chain_id, limit, offset)?;
    let json = serde_json::to_string(&blocks)?;

    resputil::ok_json(json)
}