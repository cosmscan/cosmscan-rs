use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::Insertable;
use diesel::PgConnection;
use diesel::Queryable;
use serde::{Deserialize, Serialize};

use crate::errors::DBModelError;
use crate::schema::blocks;
use crate::schema::blocks::dsl::blocks as all_blocks;

#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct Block {
    id: i32,
    chain_id: i32,
    height: i64,
    block_hash: String,
    prev_hash: String,
    proposer_address: String,
    last_commit_hash: String,
    data_hash: String,
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

impl Block {
    pub fn latest_block_height(conn: &PgConnection, chain_id: i32) -> Result<i64, DBModelError> {
        all_blocks.order(blocks::height.desc())
            .limit(1)
            .select(blocks::height)
            .first(conn)
            .map_err(|e| e.into())
    }
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[table_name = "blocks"]
pub struct NewBlock {
    chain_id: i32,
    height: i64,
    block_hash: String,
    prev_hash: String,
    proposer_address: String,
    last_commit_hash: String,
    data_hash: String,
    validators_hash: String,
    next_validators_hash: String,
    consensus_hash: String,
    app_hash: String,
    last_result_hash: String,
    evidence_hash: String,
    block_time: NaiveDateTime,
    inserted_at: NaiveDateTime,
}

impl NewBlock {
    pub fn insert(new_block: &NewBlock, conn: &PgConnection) -> Result<usize, DBModelError> {
        diesel::insert_into(blocks::table)
            .values(new_block)
            .execute(conn)
            .map_err(|e| e.into())
    }
}