use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::Insertable;
use diesel::PgConnection;
use diesel::Queryable;
use serde::{Deserialize, Serialize};

use crate::errors::DBModelError;

#[derive(Debug, Queryable, Serialize)]
pub struct Block {
    id: i32,
    chain_id: i32,
    height: i64,
    block_hash: String,
    prev_hash: String,
    proposer_address: String,
    last_commit_hash: String,
    date_hash: String,
    validators_hash: String,
    next_validators_hash: String,
    consensus_hash: String,
    app_hash: String,
    last_result_hash: String,
    evidence_hash: String,
    block_time: NaiveDateTime,
    inserted_at: NaiveDateTime,
    updated_at: Option<NaiveDateTime>,
}

