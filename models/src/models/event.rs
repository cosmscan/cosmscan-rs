use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::Insertable;
use diesel::PgConnection;
use diesel::Queryable;
use serde::{Deserialize, Serialize};

use crate::errors::DBModelError;
use crate::schema::events;

#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct Event {
    pub id: i32,
    pub chain_id: i32,
    pub tx_type: i16,
    pub tx_hash: Option<String>,
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
    pub event_type: String,
    pub event_key: String,
    pub event_value: String,
    pub indexed: bool,
    pub inserted_at: NaiveDateTime,
}

impl NewEvent {
    pub fn insert(new_event: &NewEvent, conn: &PgConnection) -> Result<usize, DBModelError> {
        diesel::insert_into(events::table)
            .values(new_event)
            .execute(conn)
            .map_err(|e| e.into())
    }
}
