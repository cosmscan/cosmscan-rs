use std::sync::Arc;

use crate::committer::Committer;
use crate::current_time;
use crate::fetchers::committed_block_fetcher::{CommittedBlockFetcher, CommittedBlockWorker};
use crate::messages::MsgCommittedBlock;
use crate::{config::Config, errors::Error};

use cosmscan_models::models::chain::{Chain, NewChain};
use cosmscan_models::storage::StorageReader;
use cosmscan_models::{
    db::BackendDB,
    storage::{PersistenceStorage, StorageWriter},
};

use log::error;
use tokio::sync::{mpsc, Mutex};

type SharedClient = Arc<Mutex<cosmos_client::client::Client>>;

/// Indexer is for fetching ABCI blocks, transactions and logs.
pub struct Indexer<T: StorageWriter + StorageReader> {
    pub config: Config,
    pub storage: T,
    pub client: SharedClient,
}

impl Indexer<PersistenceStorage<BackendDB>> {
    pub async fn new(config: Config) -> Result<Self, Error> {
        let db = BackendDB::new(config.db.clone());
        let storage = PersistenceStorage::new(db);

        // create a shared cosmos client
        let client_config = cosmos_client::client::ClientConfig {
            tendermint_rpc_endpoint: config.fetcher.tendermint_rpc_endpoint.clone(),
            grpc_endpoint: config.fetcher.grpc_endpoint.clone(),
            rest_api_endpoint: config.fetcher.rest_api_endpoint.clone(),
        };

        let client = cosmos_client::client::Client::new(client_config)
            .await
            .map(Mutex::new)
            .map(Arc::new)?;

        Ok(Indexer {
            config,
            storage,
            client,
        })
    }

    pub async fn start(&self) -> Result<(), Error> {
        let (committed_block_s, mut committed_block_r) = mpsc::channel::<MsgCommittedBlock>(100);

        // get chain info from database, if it doesn't exists, create a new one.
        let chain = self.load_chain_or_store()?;
        let start_block = match self.load_latest_block_height(&chain) {
            Some(height) => height + 1,
            None => self.config.fetcher.start_block,
        };

        let committed_block_fetcher = CommittedBlockFetcher::new(self.client.clone()).await?;
        CommittedBlockWorker::spawn(committed_block_fetcher, committed_block_s, start_block);

        // create a committer and run it
        let db_config = self.config.db.clone();
        let committer = Committer::new(db_config, chain);

        loop {
            tokio::select! {
                Some(val) = committed_block_r.recv() => {
                    committer.commit_block(val).unwrap();
                }
            }
        }
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
