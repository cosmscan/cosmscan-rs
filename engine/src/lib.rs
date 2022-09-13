use chrono::{NaiveDateTime, Utc};
use sha2::{Sha256, Digest};

pub mod app;
pub mod client;
pub mod committer;
pub mod config;
pub mod errors;
pub mod fetcher;
pub mod messages;

pub fn current_time() -> NaiveDateTime {
    NaiveDateTime::from_timestamp(Utc::now().timestamp(), 0)
}

pub fn bytes_to_tx_hash(data: impl AsRef<[u8]>) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let tx_hash = hasher.finalize();
    format!("{:X}", tx_hash)
}