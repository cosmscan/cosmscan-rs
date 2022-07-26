use std::sync::Arc;

use chrono::{NaiveDateTime, Utc};
use sha2::{Digest, Sha256};
use tokio::sync::Mutex;

pub mod committer;
pub mod config;
pub mod errors;
pub mod fetchers;
pub mod indexer;
pub mod messages;

pub type SharedClient = Arc<Mutex<cosmos_client::client::Client>>;

pub fn current_time() -> NaiveDateTime {
    NaiveDateTime::from_timestamp(Utc::now().timestamp(), 0)
}

pub fn bytes_to_tx_hash(data: impl AsRef<[u8]>) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let tx_hash = hasher.finalize();
    format!("{:X}", tx_hash)
}
