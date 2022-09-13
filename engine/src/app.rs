use crate::committer::Committer;
use crate::fetcher::Fetcher;
use crate::messages::MsgCommittedBlock;
use crate::{config::Config, errors::Error};

use cosmscan_models::storage::StorageReader;
use cosmscan_models::{
    db::BackendDB,
    storage::{PersistenceStorage, StorageWriter},
};

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
        let (tx, rv) = mpsc::channel::<MsgCommittedBlock>(100);

        let fetcher = Fetcher::new(
            self.config.fetcher.clone(),
            tx,
            u64::from(self.config.fetcher.start_block),
        )
        .await?;

        let mut committer = Committer::new(
            self.config.db.clone(),
            self.config.chain.clone(),
            rv,
            u64::from(self.config.fetcher.start_block),
        );

        tokio::spawn(async move {
            fetcher.run_fetch_loop().await.unwrap();
        });

        committer.start().await?;

        Ok(())
    }
}
