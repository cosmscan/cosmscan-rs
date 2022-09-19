use cosmscan_models::models;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub chain_id: i32,
    pub transaction_hash: String,
    pub height: i64,
    pub code: i32,
    pub code_space: String,
    pub tx_data: String,
    pub raw_log: String,
    pub info: String,
    pub memo: Option<String>,
    pub gas_wanted: i64,
    pub gas_used: i64,
    pub tx_timestamp: String,
    pub messages: Vec<serde_json::Value>,
    pub events: Vec<Event>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub tx_type: i16,
    pub tx_hash: Option<String>,
    pub block_height: i64,
    pub event_seq: i32,
    pub event_type: String,
    pub event_key: String,
    pub event_value: String,
    pub indexed: bool,
}

impl Transaction {
    pub fn new(
        tx: models::transaction::Transaction,
        events: Vec<models::event::Event>,
        messages: Vec<models::message::Message>,
    ) -> Self {
        let tx_events = events
            .into_iter()
            .map(|e| Event {
                tx_type: e.tx_type,
                tx_hash: e.tx_hash,
                block_height: e.block_height,
                event_seq: e.event_seq,
                event_type: e.event_type,
                event_key: e.event_key,
                event_value: e.event_value,
                indexed: e.indexed,
            })
            .collect::<Vec<_>>();

        let tx_messages = messages.into_iter().map(|m| m.rawdata).collect::<Vec<_>>();

        Self {
            chain_id: tx.chain_id,
            transaction_hash: tx.transaction_hash,
            height: tx.height,
            code: tx.code,
            code_space: tx.code_space,
            tx_data: tx.tx_data,
            raw_log: tx.raw_log,
            info: tx.info,
            memo: tx.memo,
            gas_wanted: tx.gas_wanted,
            gas_used: tx.gas_used,
            tx_timestamp: tx.tx_timestamp,
            messages: tx_messages,
            events: tx_events,
        }
    }
}
