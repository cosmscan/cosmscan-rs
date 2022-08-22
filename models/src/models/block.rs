use chrono::NaiveDateTime;
use diesel::Insertable;
use diesel::Queryable;
use serde::{Deserialize, Serialize};

use crate::schema::blocks;

#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct Block {
    pub id: i32,
    pub chain_id: i32,
    pub height: i64,
    pub block_hash: String,
    pub prev_hash: String,
    pub proposer_address: String,
    pub last_commit_hash: String,
    pub data_hash: String,
    pub validators_hash: String,
    pub next_validators_hash: String,
    pub consensus_hash: String,
    pub app_hash: String,
    pub last_result_hash: String,
    pub evidence_hash: String,
    pub block_time: NaiveDateTime,
    pub inserted_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[table_name = "blocks"]
pub struct NewBlock {
    pub chain_id: i32,
    pub height: i64,
    pub block_hash: String,
    pub prev_hash: String,
    pub proposer_address: String,
    pub last_commit_hash: String,
    pub data_hash: String,
    pub validators_hash: String,
    pub next_validators_hash: String,
    pub consensus_hash: String,
    pub app_hash: String,
    pub last_result_hash: String,
    pub evidence_hash: String,
    pub block_time: NaiveDateTime,
    pub inserted_at: NaiveDateTime,
}
