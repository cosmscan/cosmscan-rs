use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::Insertable;
use diesel::PgConnection;
use diesel::Queryable;
use serde::{Deserialize, Serialize};

use crate::errors::Error;
use crate::schema::transactions;

#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct Transaction {
    pub id: i32,
    pub chain_id: i32,
    pub transaction_hash: String,
    pub height: i64,
    pub code: i32,
    pub code_space: String,
    pub tx_data: Vec<u8>,
    pub raw_log: Option<String>,
    pub info: Option<String>,
    pub memo: Option<String>,
    pub gas_wanted: i64,
    pub gas_used: i64,
    pub tx_date: Option<String>,
    pub inserted_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

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

impl NewTransaction {
    pub fn insert(conn: &PgConnection, new_tx: &NewTransaction) -> Result<usize, Error> {
        diesel::insert_into(transactions::table)
            .values(new_tx)
            .execute(conn)
            .map_err(|e| e.into())
    }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use serial_test::serial;

    use crate::{
        config::DBConfig,
        db::{BackendDB, Database},
        models::test_helpers::cleanup_db,
    };

    use super::*;

    #[test]
    #[serial]
    fn test_insert_tx() {
        let mut db = BackendDB::new(DBConfig::default());
        let connected = db.connect();
        assert_eq!(connected, true);
        cleanup_db(&db.conn().unwrap());

        let result = NewTransaction::insert(
            &db.conn().unwrap(),
            &NewTransaction {
                chain_id: 1,
                transaction_hash: "0000txhash".to_string(),
                height: 25,
                code: 1,
                code_space: "0000codespace".to_string(),
                tx_data: "tx_data".to_string(),
                raw_log: "raw_log".to_string(),
                info: "".to_string(),
                memo: None,
                gas_wanted: 25000,
                gas_used: 23000,
                tx_timestamp: "time".to_string(),
                inserted_at: NaiveDateTime::from_timestamp(Utc::now().timestamp(), 0),
            },
        );
        assert_eq!(result.is_err(), false);
    }
}
