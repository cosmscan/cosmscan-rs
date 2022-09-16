use diesel::Insertable;
use diesel::Queryable;
use serde::{Deserialize, Serialize};

use crate::schema::messages;

#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct Message {
    pub id: i32,
    pub transaction_id: i32,
    pub seq: i32,
    pub rawdata: serde_json::Value,
    pub inserted_at: chrono::NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[table_name = "messages"]
pub struct NewMessage {
    pub transaction_id: i32,
    pub seq: i32,
    pub rawdata: serde_json::Value,
    pub inserted_at: chrono::NaiveDateTime,
}
