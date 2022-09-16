use chrono::NaiveDateTime;
use diesel::Insertable;
use diesel::Queryable;
use serde::{Deserialize, Serialize};

use crate::schema::transactions;

#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct Transaction {
    pub id: i32,
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
    pub inserted_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

// transactions (id) {
//     id -> Int4,
//     chain_id -> Int4,
//     transaction_hash -> Varchar,
//     height -> Int8,
//     code -> Int4,
//     code_space -> Varchar,
//     tx_data -> Text,
//     raw_log -> Text,
//     info -> Text,
//     memo -> Nullable<Varchar>,
//     gas_wanted -> Int8,
//     gas_used -> Int8,
//     tx_timestamp -> Varchar,
//     inserted_at -> Timestamp,
//     updated_at -> Nullable<Timestamp>,
// }

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[table_name = "transactions"]
pub struct NewTransaction {
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
    pub inserted_at: NaiveDateTime,
}
