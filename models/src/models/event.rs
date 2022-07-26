use chrono::NaiveDateTime;
use diesel::Insertable;
use diesel::Queryable;
use serde::{Deserialize, Serialize};

use crate::schema::events;

pub const TX_TYPE_TRANSACTION: i16 = 1;
pub const TX_TYPE_BEGIN_BLOCK: i16 = 2;
pub const TX_TYPE_END_BLOCK: i16 = 3;

#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct Event {
    pub id: i32,
    pub chain_id: i32,
    pub tx_type: i16,
    pub tx_hash: Option<String>,
    pub block_height: i64,
    pub event_seq: i32,
    pub event_type: String,
    pub event_key: String,
    pub event_value: String,
    pub indexed: bool,
    pub inserted_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[table_name = "events"]
pub struct NewEvent {
    pub chain_id: i32,
    pub tx_type: i16,
    pub tx_hash: Option<String>,
    pub block_height: i64,
    pub event_seq: i32,
    pub event_type: String,
    pub event_key: String,
    pub event_value: String,
    pub indexed: bool,
    pub inserted_at: NaiveDateTime,
}
