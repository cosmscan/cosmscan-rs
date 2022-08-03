use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::Insertable;
use diesel::PgConnection;
use diesel::Queryable;
use serde::{Deserialize, Serialize};

use crate::errors::DBModelError;
use crate::schema::transactions;

#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct Transaction {
    id: i32,
    chain_id: i32,
    transaction_hash: String,
    height: i64,
    code: Option<i32>,
    code_space: Option<String>,
    tx_data: Option<String>,
    raw_log: Option<String>,
    info: Option<String>,
    memo: Option<String>,
    gas_wanted: i64,
    gas_used: i64,
    tx_date: Option<String>,
    inserted_at: NaiveDateTime,
    updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[table_name = "transactions"]
pub struct NewTransaction {
    chain_id: i32,
    transaction_hash: String,
    height: i64,
    code: Option<i32>,
    code_space: Option<String>,
    tx_data: Option<String>,
    raw_log: Option<String>,
    info: Option<String>,
    memo: Option<String>,
    gas_wanted: i64,
    gas_used: i64,
    tx_date: Option<String>,
    inserted_at: NaiveDateTime,
}

impl NewTransaction {
    pub fn insert(conn: &PgConnection, new_tx: &NewTransaction) -> Result<usize, DBModelError> {
        diesel::insert_into(transactions::table)
            .values(new_tx)
            .execute(conn)
            .map_err(|e| e.into())
    }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;

    use crate::{
        config::DBConfig,
        db::{BackendDB, Database},
        models::test_helpers::cleanup_db,
    };

    use super::*;

    #[test]
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
                code: Some(1),
                code_space: Some("0000codespace".to_string()),
                tx_data: None,
                raw_log: Some("{ raw_logs }".to_string()),
                info: None,
                memo: None,
                gas_wanted: 25000,
                gas_used: 23000,
                tx_date: None,
                inserted_at: NaiveDateTime::from_timestamp(Utc::now().timestamp(), 0),
            },
        );
        assert_eq!(result.is_err(), false);
    }
}
