use crate::{
    db::{BackendDB, Database},
    errors::Error,
    models::{
        block::NewBlock,
        chain::{Chain, NewChain},
        event::NewEvent,
        message::{Message, NewMessage},
        transaction::NewTransaction,
    },
    schema::{blocks, chains, events, messages, transactions},
};

use crate::schema::blocks::dsl::blocks as all_blocks;
use crate::schema::chains::dsl::chains as all_chains;
use crate::schema::events::dsl::events as all_events;
use crate::schema::messages::dsl::messages as all_messages;
use crate::schema::transactions::dsl::transactions as all_transactions;

use crate::models::block::Block;
use crate::models::event::Event;
use crate::models::transaction::Transaction;
use diesel::{prelude::*, r2d2::ConnectionManager};
use r2d2::PooledConnection;

type Connection = PooledConnection<ConnectionManager<PgConnection>>;

/// StorageReader defines a set of methods for reading the database
pub trait StorageReader {
    // block operations
    fn find_block_by_height(&self, chain_id: i32, height: i64) -> Result<Block, Error>;
    fn list_blocks(&self, chain_id: i32, limit: i64, offset: i64) -> Result<Vec<Block>, Error>;
    fn find_latest_block(&self, chain_id: i32) -> Result<Block, Error>;

    // chain operations
    fn find_by_chain_id(&self, chain_id: String) -> Result<Chain, Error>;
    fn all_chains(&self) -> Result<Vec<Chain>, Error>;

    // trasnaction operations
    fn list_transactions(
        &self,
        chain_id: i32,
        block_height: i64,
    ) -> Result<Vec<Transaction>, Error>;
    fn find_transaction_by_hash(&self, tx_hash: String) -> Result<Transaction, Error>;

    // message operations
    fn list_messages_by_tx(&self, tx_id: i32) -> Result<Vec<Message>, Error>;

    // event operations
    fn list_events_by_tx(&self, tx_hash: String) -> Result<Vec<Event>, Error>;
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
    fn insert_transaction(&self, transaction: &NewTransaction) -> Result<Transaction, Error>;

    // message operations
    fn insert_message(&self, message: &NewMessage) -> Result<usize, Error>;
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

    fn insert_transaction(&self, transaction: &NewTransaction) -> Result<Transaction, Error> {
        let conn = self.get_conn()?;
        diesel::insert_into(transactions::table)
            .values(transaction)
            .get_result::<Transaction>(&conn)
            .map_err(|e| e.into())
    }

    fn insert_message(&self, message: &NewMessage) -> Result<usize, Error> {
        let conn = self.get_conn()?;
        diesel::insert_into(messages::table)
            .values(message)
            .execute(&conn)
            .map_err(|e| e.into())
    }
}

impl StorageReader for PersistenceStorage<BackendDB> {
    fn find_block_by_height(&self, chain_id: i32, height: i64) -> Result<Block, Error> {
        let conn = self.get_conn()?;
        all_blocks
            .filter(blocks::chain_id.eq(chain_id).and(blocks::height.eq(height)))
            .first(&conn)
            .map_err(|e| e.into())
    }

    fn list_blocks(&self, chain_id: i32, limit: i64, offset: i64) -> Result<Vec<Block>, Error> {
        let conn = self.get_conn()?;
        all_blocks
            .filter(blocks::chain_id.eq(chain_id))
            .order(blocks::height.desc())
            .limit(limit)
            .offset(offset)
            .load::<Block>(&conn)
            .map_err(|e| e.into())
    }

    fn find_latest_block(&self, chain_id: i32) -> Result<Block, Error> {
        let conn = self.get_conn()?;
        all_blocks
            .filter(blocks::chain_id.eq(chain_id))
            .order(blocks::height.desc())
            .first(&conn)
            .map_err(|e| e.into())
    }

    fn find_by_chain_id(&self, chain_id: String) -> Result<Chain, Error> {
        let conn = self.get_conn()?;
        all_chains
            .filter(chains::chain_id.eq(chain_id))
            .first(&conn)
            .map_err(|e| e.into())
    }

    fn all_chains(&self) -> Result<Vec<Chain>, Error> {
        let conn = self.get_conn()?;
        all_chains.load::<Chain>(&conn).map_err(|e| e.into())
    }

    fn list_transactions(&self, chain_id: i32, height: i64) -> Result<Vec<Transaction>, Error> {
        let conn = self.get_conn()?;
        let condition = transactions::height
            .eq(height)
            .and(transactions::chain_id.eq(chain_id));
        all_transactions
            .filter(condition)
            .load::<Transaction>(&conn)
            .map_err(|e| e.into())
    }

    fn find_transaction_by_hash(&self, tx_hash: String) -> Result<Transaction, Error> {
        let conn = self.get_conn()?;
        all_transactions
            .filter(transactions::transaction_hash.eq(tx_hash))
            .first(&conn)
            .map_err(|e| e.into())
    }

    fn list_messages_by_tx(&self, tx_id: i32) -> Result<Vec<Message>, Error> {
        let conn = self.get_conn()?;
        all_messages
            .filter(messages::transaction_id.eq(tx_id))
            .load::<Message>(&conn)
            .map_err(|e| e.into())
    }

    fn list_events_by_tx(&self, tx_hash: String) -> Result<Vec<Event>, Error> {
        let conn = self.get_conn()?;
        all_events
            .filter(events::tx_hash.eq(tx_hash))
            .load::<Event>(&conn)
            .map_err(|e| e.into())
    }
}
