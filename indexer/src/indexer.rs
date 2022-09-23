use crate::committer::Committer;
use crate::current_time;
use crate::fetcher::Fetcher;
use crate::messages::MsgCommittedBlock;
use crate::{config::Config, errors::Error};

use cosmscan_models::models::chain::{Chain, NewChain};
use cosmscan_models::storage::StorageReader;
use cosmscan_models::{
    db::BackendDB,
    storage::{PersistenceStorage, StorageWriter},
};

use log::error;
use tokio::sync::mpsc;

/// Indexer is for fetching ABCI blocks, transactions and logs.
pub struct Indexer<T: StorageWriter + StorageReader> {
    pub config: Config,
    pub storage: T,
}

impl Indexer<PersistenceStorage<BackendDB>> {
    pub async fn new(config: Config) -> Result<Self, Error> {
        let db = BackendDB::new(config.db.clone());
        let storage = PersistenceStorage::new(db);

        Ok(Indexer { config, storage })
    }

    pub async fn start(&self) -> Result<(), Error> {
        let (committed_block_s, mut committed_block_r) = mpsc::channel::<MsgCommittedBlock>(100);

        // get chain info from database, if it doesn't exists, create a new one.
        let chain = self.load_chain_or_store()?;
        let start_block = match self.load_latest_block_height(&chain) {
            Some(height) => height + 1,
            None => self.config.fetcher.start_block,
        };

        // create a fetcher and run it
        let fetcher_config = self.config.fetcher.clone();
        let fetcher = Fetcher::new(fetcher_config, committed_block_s, start_block).await?;

        tokio::spawn(async move {
            fetcher.run().await.unwrap();
            panic!("fetcher is stopped unexpectedly");
        });

        // create a committer and run it
        let db_config = self.config.db.clone();
        let committer = Committer::new(db_config, chain);

        tokio::select! {
            Some(val) = committed_block_r.recv() => {
                committer.commit_block(val).unwrap();
            }
        }

        Ok(())
    }

    // load chain info from storage
    // if it doesn't exists we'll create a new chain in the storage.
    pub fn load_chain_or_store(&self) -> Result<Chain, Error> {
        match self
            .storage
            .find_by_chain_id(self.config.chain.chain_id.clone())
        {
            Ok(chain) => {
                return Ok(chain);
            }
            Err(e) => {
                // match with not found error
                match e {
                    cosmscan_models::errors::Error::NotFound => {
                        // create a new chain info
                        let new_chain_id = self.storage.insert_chain(&NewChain {
                            chain_id: self.config.chain.chain_id.clone(),
                            chain_name: self.config.chain.chain_name.clone(),
                            inserted_at: current_time(),
                        })?;
                        return Ok(Chain {
                            id: new_chain_id as i32,
                            chain_id: self.config.chain.chain_id.clone(),
                            chain_name: self.config.chain.chain_name.clone(),
                            icon_url: None,
                            webisite: None,
                            inserted_at: current_time(),
                            updated_at: None,
                        });
                    }
                    _ => {
                        error!("unexpected error: {:?}", e);
                        return Err(Error::DBError(e));
                    }
                }
            }
        };
    }

    pub fn load_latest_block_height(&self, chain: &Chain) -> Option<i64> {
        match self.storage.latest_block_height(chain.id) {
            Ok(height) => Some(height),
            Err(_) => None,
        }
    }
}
