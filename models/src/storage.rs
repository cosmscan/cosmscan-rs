use crate::{
    models::{block::NewBlock, chain::{NewChain, Chain}, event::NewEvent, transaction::NewTransaction}, db::{BackendDB, Database}, errors::Error, schema::{blocks, chains, events, transactions},
};

use crate::schema::chains::dsl::chains as all_chains;

use diesel::{prelude::*, r2d2::ConnectionManager};
use r2d2::PooledConnection;
use crate::models::block::Block;
use crate::models::event::Event;
use crate::models::transaction::Transaction;

type Connection = PooledConnection<ConnectionManager<PgConnection>>;

/// StorageReader defines a set of methods for reading the database
pub trait StorageReader {
    // simple count function
    fn count_blocks(&self) -> Result<i64, Error>;
    fn count_chains(&self) -> Result<i64, Error>;
    fn count_events(&self) -> Result<i64, Error>;
    fn count_transactions(&self) -> Result<i64, Error>;

    fn find_by_chain_id(&self, chain_id: String) -> Result<Chain, Error>;
    fn all_chains(&self) -> Result<Vec<Chain>, Error>;
    fn list_blocks(&self, chain_id: i32, limit: i64, offset: i64) -> Result<Vec<Block>, Error>;
    fn list_transactions(&self, chain_id: i32, block_number: i64) -> Result<Vec<(Transaction, Vec<Event>)>, Error>;
}

/// StorageWriter defines a set of method for writing/updating the database.
pub trait StorageWriter {
    // block operations
    fn insert_block(&self, block: &NewBlock) -> Result<usize, Error>;
    fn latest_block_height(&self, chain_id: i32) -> Result<i64, Error>;

    // chain operations
    fn insert_chain(&self, chain: &NewChain) -> Result<usize, Error>;

    // event operations
    fn insert_event(&self, event: &NewEvent) -> Result<usize, Error>;

    // transaction operations
    fn insert_transaction(&self, transaction: &NewTransaction) -> Result<usize, Error>;
}

/// PersistenceStorage should implements both [`StorageWriter`] and [`StorageReader`]
pub struct PersistenceStorage<T>
where T: Database {
     db: T,
}

impl PersistenceStorage<BackendDB> {
    pub fn new(mut db: BackendDB) -> Self {
        let connected = db.connect();
        if !connected {
            panic!("failed to connect to the database, please check configuration")
        }
    
        Self { db }
    }

    pub fn get_conn(&self) -> Result<Connection, Error> {
        match self.db.conn() {
            Some(conn) => {
                Ok(conn)
            }
            None => {
                Err(Error::ClientDoesntExists)
            }
        }
    }

    pub fn within_transaction<F> (&self, f: F) -> Result<bool, Error>
    where F: FnOnce() -> Result<bool, Error> {
        let conn = self.get_conn()?;
        conn.build_transaction()
            .repeatable_read()
            .run::<bool, Error, _>(|| {
                f()
            })
    }
}

impl StorageWriter for PersistenceStorage<BackendDB>{
    fn insert_block(&self, block: &NewBlock) -> Result<usize, Error> {
        let conn = self.get_conn()?;
        diesel::insert_into(blocks::table)
            .values(block)
            .execute(&conn)
            .map_err(|e| e.into())
    }

    fn latest_block_height(&self, chain_id: i32) -> Result<i64, Error> {
        let conn = self.get_conn()?;
        blocks::table
            .select(blocks::height)
            .filter(blocks::chain_id.eq(chain_id))
            .order(blocks::height.desc())
            .limit(1)
            .first(&conn)
            .map_err(|e| e.into())
    }

    fn insert_chain(&self, chain: &NewChain) -> Result<usize, Error> {
        let conn = self.get_conn()?;
        diesel::insert_into(chains::table)
            .values(chain)
            .execute(&conn)
            .map_err(|e| e.into())
    }

    fn insert_event(&self, event: &NewEvent) -> Result<usize, Error> {
        let conn = self.get_conn()?;
        diesel::insert_into(events::table)
            .values(event)
            .execute(&conn)
            .map_err(|e| e.into())
    }

    fn insert_transaction(&self, transaction: &NewTransaction) -> Result<usize, Error> {
        let conn = self.get_conn()?;
        diesel::insert_into(transactions::table)
            .values(transaction)
            .execute(&conn)
            .map_err(|e| e.into())
    }
}

impl StorageReader for PersistenceStorage<BackendDB> {
    fn count_blocks(&self) -> Result<i64, Error> {
        todo!()
    }

    fn count_chains(&self) -> Result<i64, Error> {
        todo!()
    }

    fn count_events(&self) -> Result<i64, Error> {
        todo!()
    }

    fn count_transactions(&self) -> Result<i64, Error> {
        todo!()
    }

    fn find_by_chain_id(&self, chain_id: String) -> Result<Chain, Error> {
        let conn = self.get_conn()?;
        all_chains
            .filter(chains::chain_id.eq(chain_id))
            .first(&conn)
            .map_err(|e| e.into())
    }

    fn all_chains(&self) -> Result<Vec<Chain>, Error> {
        todo!()
    }

    fn list_blocks(&self, chain_id: i32, limit: i64, offset: i64) -> Result<Vec<Block>, Error> {
        todo!()
    }

    fn list_transactions(&self, chain_id: i32, block_number: i64) -> Result<Vec<(Transaction, Vec<Event>)>, Error> {
        todo!()
    }
}

/// Unit test codes for Storage database
/// We don't use mock object, so you should run your own postgreSQL database.
/// But, we provided a sample database for you to use. Please run the following commands.
/// `docker-compose -f docker-compose.db.yml up`
#[cfg(test)]
mod tests {
    use diesel::backend::Backend;
    use serial_test::serial;

    use crate::config::DBConfig;
    use crate::db::{BackendDB, Database};
    use crate::models::block::NewBlock;
    use diesel::prelude::*;
    use crate::schema::blocks::dsl::blocks as all_blocks;
    use crate::schema::chains::dsl::chains as all_chains;
    use crate::schema::events::dsl::events as all_events;
    use crate::schema::transactions::dsl::transactions as all_transactions;
    use crate::storage::StorageWriter;
    use crate::utils::current_time;

    use super::{PersistenceStorage};

    fn cleanup_db(db: &PersistenceStorage<BackendDB>) {
        let conn = db.get_conn().unwrap();

        diesel::delete(all_chains)
            .execute(&conn)
            .expect("failed to cleanup database");

        diesel::delete(all_transactions)
            .execute(&conn)
            .expect("failed to cleanup database");

        diesel::delete(all_blocks)
            .execute(&conn)
            .expect("failed to cleanup database");

        diesel::delete(all_events)
            .execute(&conn)
            .expect("failed to cleanup database");
    }

    fn sample_block(chain_id: i32, height: i64) -> NewBlock {
        NewBlock {
            chain_id,
            height,
            block_hash: String::from("block_hash"),
            prev_hash: String::from("prev_hash"),
            proposer_address: String::from("proposer_address"),
            last_commit_hash: String::from("last_commit_hash"),
            data_hash: String::from("data_hash"),
            validators_hash: String::from("validator_hash"),
            next_validators_hash: String::from("next_validators_hash"),
            consensus_hash: String::from("consensus_hash"),
            app_hash: String::from("app_hash"),
            last_result_hash: String::from("last_result_hash"),
            evidence_hash: String::from("evidence_hash"),
            block_time: current_time(),
            inserted_at: current_time()
        }
    }

    #[test]
    #[serial]
    fn insert_block() {
        let db = BackendDB::new(DBConfig::default());
        let storage = PersistenceStorage::new(db);
        cleanup_db(&storage);

        // test insert block
        let new_block = sample_block(1, 1);
        let result = storage.insert_block(&new_block);
        assert!(result.is_ok());
    }
}