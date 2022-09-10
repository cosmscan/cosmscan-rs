use crate::{
    db::{BackendDB, Database},
    errors::Error,
    models::{
        block::NewBlock,
        chain::{Chain, NewChain},
        event::NewEvent,
        transaction::NewTransaction,
    },
    schema::{blocks, chains, events, transactions},
};

use crate::schema::chains::dsl::chains as all_chains;

use crate::models::block::Block;
use crate::models::event::Event;
use crate::models::transaction::Transaction;
use diesel::{prelude::*, r2d2::ConnectionManager};
use r2d2::PooledConnection;

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
    fn list_transactions(
        &self,
        chain_id: i32,
        block_number: i64,
    ) -> Result<Vec<(Transaction, Vec<Event>)>, Error>;
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
where
    T: Database,
{
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
            Some(conn) => Ok(conn),
            None => Err(Error::ClientDoesntExists),
        }
    }

    pub fn within_transaction<F>(&self, f: F) -> Result<bool, Error>
    where
        F: FnOnce() -> Result<bool, Error>,
    {
        let conn = self.get_conn()?;
        conn.build_transaction()
            .repeatable_read()
            .run::<bool, Error, _>(|| f())
    }
}

impl StorageWriter for PersistenceStorage<BackendDB> {
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

    fn list_blocks(&self, _chain_id: i32, _limit: i64, _offset: i64) -> Result<Vec<Block>, Error> {
        todo!()
    }

    fn list_transactions(
        &self,
        _chain_id: i32,
        _block_number: i64,
    ) -> Result<Vec<(Transaction, Vec<Event>)>, Error> {
        todo!()
    }
}
