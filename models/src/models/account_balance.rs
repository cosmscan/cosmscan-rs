use diesel::Insertable;
use diesel::Queryable;
use serde::{Deserialize, Serialize};

use crate::schema::account_balance;

#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct Account {
    pub id: i32,
    pub account_id: i32,
    pub amount: i64,
    pub denom: String,
    pub inserted_at: chrono::NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[table_name = "account_balance"]
pub struct NewAccount {
    pub account_id: i32,
    pub amount: i64,
    pub denom: String,
    pub inserted_at: chrono::NaiveDateTime,
}
