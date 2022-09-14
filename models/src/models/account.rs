use serde::{Serialize, Deserialize};
use diesel::Insertable;
use diesel::Queryable;

use crate::schema::accounts;

#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct Account {
    pub id: i32,
    pub chain_id: i32,
    pub address: String,
    pub inserted_at: chrono::NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[table_name = "accounts"]
pub struct NewAccount {
    pub chain_id: i32,
    pub address: String,
    pub inserted_at: chrono::NaiveDateTime,
}