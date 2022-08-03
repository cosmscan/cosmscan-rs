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
    id: i32,
    chain_id: i32,
    tx_type: i16,
    tx_hash: Option<String>,
    event_type: String,
    event_key: String,
    event_value: String,
    indexed: bool,
    inserted_at: NaiveDateTime,
    updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[table_name = "events"]
pub struct NewEvent {
    chain_id: i32,
    tx_type: i16,
    tx_hash: Option<String>,
    event_type: String,
    event_key: String,
    event_value: String,
    indexed: bool,
    inserted_at: NaiveDateTime,
}

impl NewEvent {
    pub fn insert(new_event: &NewEvent, conn: &PgConnection) -> Result<usize, DBModelError> {
        diesel::insert_into(events::table)
            .values(new_event)
            .execute(conn)
            .map_err(|e| e.into())
    }
}
