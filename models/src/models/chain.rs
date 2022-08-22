use chrono::NaiveDateTime;
use diesel::Insertable;
use diesel::Queryable;
use serde::{Deserialize, Serialize};

use crate::schema::chains;

#[derive(Debug, Queryable, Serialize)]
pub struct Chain {
    pub id: i32,
    pub chain_id: String,
    pub chain_name: String,
    pub icon_url: Option<String>,
    pub webisite: Option<String>,
    pub inserted_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Deserialize, Serialize)]
#[table_name = "chains"]
pub struct NewChain {
    pub chain_id: String,
    pub chain_name: String,
    pub inserted_at: NaiveDateTime,
}