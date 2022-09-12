use std::str::from_utf8;

use chrono::{NaiveDateTime, Utc};
use cosmos_sdk_proto::cosmos::tx::v1beta1::GetTxResponse;
use cosmoscout_models::models::event::{
    NewEvent, TX_TYPE_BEGIN_BLOCK, TX_TYPE_END_BLOCK, TX_TYPE_TRANSACTION,
};
use sha2::{Digest, Sha256};
use tendermint_rpc::endpoint::block_results;

pub fn current_time() -> NaiveDateTime {
    NaiveDateTime::from_timestamp(Utc::now().timestamp(), 0)
}

/// extract beging block events from block_results
pub fn extract_begin_block_events(
    block_results: &block_results::Response,
    chain_id: i32,
    current_time: &NaiveDateTime,
) -> Vec<NewEvent> {
    if let Some(events) = &block_results.begin_block_events {
        events
            .iter()
            .flat_map(|event| {
                let mut result: Vec<NewEvent> = vec![];

                for attr in event.attributes.iter() {
                    result.push(NewEvent {
                        chain_id,
                        tx_type: TX_TYPE_BEGIN_BLOCK,
                        tx_hash: None,
                        event_type: event.type_str.clone(),
                        event_key: attr.key.to_string(),
                        event_value: attr.value.to_string(),
                        indexed: false,
                        inserted_at: *current_time,
                    });
                }

                result
            })
            .collect::<Vec<_>>()
    } else {
        vec![]
    }
}

/// extract end block events from block_results
pub fn extract_end_block_events(
    block_results: &block_results::Response,
    chain_id: i32,
    current_time: &NaiveDateTime,
) -> Vec<NewEvent> {
    if let Some(events) = &block_results.end_block_events {
        events
            .iter()
            .flat_map(|event| {
                let mut result: Vec<NewEvent> = vec![];

                for attr in event.attributes.iter() {
                    result.push(NewEvent {
                        chain_id,
                        tx_type: TX_TYPE_END_BLOCK,
                        tx_hash: None,
                        event_type: event.type_str.clone(),
                        event_key: attr.key.to_string(),
                        event_value: attr.value.to_string(),
                        indexed: false,
                        inserted_at: *current_time,
                    });
                }

                result
            })
            .collect::<Vec<_>>()
    } else {
        vec![]
    }
}

/// extract events from transaction
pub fn extract_tx_events(
    tx: &GetTxResponse,
    chain_id: i32,
    current_time: &NaiveDateTime,
) -> Vec<NewEvent> {
    let response = tx.tx_response.as_ref().unwrap();

    response
        .events
        .iter()
        .flat_map(|event| {
            let mut result: Vec<NewEvent> = vec![];

            for attr in event.attributes.iter() {
                result.push(NewEvent {
                    chain_id,
                    tx_type: TX_TYPE_TRANSACTION,
                    tx_hash: Some(response.txhash.clone()),
                    event_type: event.r#type.clone(),
                    event_key: from_utf8(&attr.key).unwrap().to_string(),
                    event_value: from_utf8(&attr.value).unwrap().to_string(),
                    indexed: false,
                    inserted_at: *current_time,
                });
            }

            result
        })
        .collect::<Vec<_>>()
}

/// convert data representation to transaction hash
pub fn bytes_to_tx_hash(data: impl AsRef<[u8]>) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let tx_hash = hasher.finalize();
    format!("{:X}", tx_hash)
}