use crate::{
    models::{block::NewBlock, chain::NewChain, event::NewEvent, transaction::NewTransaction}, db::{BackendDB, Database}, errors::Error, schema::{blocks, chains, events, transactions},
};

use diesel::{prelude::*, r2d2::ConnectionManager};
use r2d2::PooledConnection;

pub trait Storage {
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

struct PersistenceStorage<T>
where T: Database {
     db: T,
}

impl PersistenceStorage<BackendDB> {
    pub fn new(mut db: BackendDB) -> Self {
        let _ = db.connect();
    
        Self { db }
    }

    pub fn get_conn(&self) -> Result<PooledConnection<ConnectionManager<PgConnection>>, Error> {
        match self.db.conn() {
            Some(conn) => {
                Ok(conn)
            }
            None => {
                Err(Error::ClientDoesntExists)
            }
        }
    }
}

impl Storage for PersistenceStorage<BackendDB>{
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

#[cfg(test)]
mod tests {
    use serial_test::serial;

    use crate::db::{BackendDB, Database};
    use diesel::prelude::*;
    use crate::schema::blocks::dsl::blocks as all_blocks;
    use crate::schema::chains::dsl::chains as all_chains;
    use crate::schema::events::dsl::events as all_events;
    use crate::schema::transactions::dsl::transactions as all_transactions;

    fn cleanup_db(db: &mut BackendDB) {
        let conn = db.conn().unwrap();

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

    #[test]
    #[serial]
    fn insert_block() {
        
    }
}