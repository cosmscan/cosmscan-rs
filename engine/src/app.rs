use crate::fetcher::{Fetcher, MsgCommittedBlock};
use crate::{config::Config, errors::Error};

use cosmoscout_models::storage::StorageReader;
use cosmoscout_models::{
    db::BackendDB,
    storage::{PersistenceStorage, StorageWriter},
};
use log::info;
use tokio::sync::mpsc;

/// App is for fetching ABCI blocks, transactions and logs.
pub struct App<T: StorageWriter + StorageReader> {
    pub config: Config,
    pub storage: T,
}

impl App<PersistenceStorage<BackendDB>> {
    pub async fn new(config: Config) -> Result<Self, Error> {
        let db = BackendDB::new(config.db.clone());
        let storage = PersistenceStorage::new(db);

        Ok(App { config, storage })
    }

    pub async fn start(&self) -> Result<(), Error> {
        let (tx, mut rv) = mpsc::channel::<MsgCommittedBlock>(100);

        let fetcher = Fetcher::new(
            self.config.fetcher.clone(),
            tx,
            u64::from(self.config.fetcher.start_block),
        )
        .await?;

        tokio::spawn(async move {
            fetcher.run_fetch_loop().await.unwrap();
        });

        while let Some(msg) = rv.recv().await {
            info!("received committed block: {:?}", msg.block.block_hash);
        }

        Ok(())
    }
}
