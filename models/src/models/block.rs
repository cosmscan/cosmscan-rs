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
        all_blocks
            .order(blocks::height.desc())
            .limit(1)
            .filter(blocks::chain_id.eq(chain_id))
            .select(blocks::height)
            .first(conn)
            .map_err(|e| e.into())
    }
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
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
    pub fn insert(conn: &PgConnection, new_block: &NewBlock) -> Result<usize, DBModelError> {
        diesel::insert_into(blocks::table)
            .values(new_block)
            .execute(conn)
            .map_err(|e| e.into())
    }
}

#[cfg(test)]
mod tests {
    use chrono::{NaiveDateTime, Utc};

    use crate::{
        config::DBConfig,
        db::{BackendDB, Database},
        models::{
            block::{Block, NewBlock},
            test_helpers::cleanup_db,
        },
    };

    #[test]
    fn insert_new_block() {
        let mut db = BackendDB::new(DBConfig::default());
        let connected = db.connect();
        assert_eq!(connected, true);
        cleanup_db(&db.conn().unwrap());

        let mut new_block = NewBlock {
            chain_id: 1,
            height: 1,
            block_hash: "foobar".to_string(),
            prev_hash: "foobar".to_string(),
            proposer_address: "foobar".to_string(),
            last_commit_hash: "foobar".to_string(),
            data_hash: "foobar".to_string(),
            validators_hash: "foobar".to_string(),
            next_validators_hash: "foobar".to_string(),
            consensus_hash: "foobar".to_string(),
            app_hash: "foobar".to_string(),
            last_result_hash: "foobar".to_string(),
            evidence_hash: "foobar".to_string(),
            block_time: NaiveDateTime::from_timestamp(Utc::now().timestamp(), 0),
            inserted_at: NaiveDateTime::from_timestamp(Utc::now().timestamp(), 0),
        };

        let result = NewBlock::insert(&db.conn().unwrap(), &new_block.clone());
        assert_eq!(result.is_err(), false);

        // fetch latest block
        let latest_height = Block::latest_block_height(&db.conn().unwrap(), 1).unwrap();
        assert_eq!(latest_height, 1);

        // increase block number
        new_block.height = 2;
        let result = NewBlock::insert(&db.conn().unwrap(), &new_block);
        assert_eq!(result.is_err(), false);

        let latest_height = Block::latest_block_height(&db.conn().unwrap(), 1).unwrap();
        assert_eq!(latest_height, 2);
    }
}
