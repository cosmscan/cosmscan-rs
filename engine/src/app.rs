use crate::{
    config::{ChainConfig, Config},
    convert::{NewBlockSchema, NewTxSchema},
    errors::Error,
    utils::{
        current_time, extract_begin_block_events, extract_end_block_events, extract_tx_events,
    },
};

use cosmos_sdk_proto::cosmos::tx::v1beta1::{
    service_client::ServiceClient, GetTxRequest, GetTxResponse,
};
use cosmoscout_models::storage::StorageReader;
use cosmoscout_models::{
    db::BackendDB,
    models::{block::NewBlock, chain::NewChain, transaction::NewTransaction},
    storage::{PersistenceStorage, StorageWriter},
};
use futures::future;
use log::{debug, error, info, warn};
use sha2::{Digest, Sha256};
use std::sync::Arc;
use std::time::Duration;
use tendermint::block::Height;
use tendermint_rpc::{
    endpoint::{block, block_results},
    Client, HttpClient,
};
use tokio::{sync::Mutex, time::sleep};
use tonic::transport::Channel;

/// App is for fetching ABCI blocks, transactions and logs.
pub struct App<T: StorageWriter + StorageReader> {
    pub config: Config,
    pub storage: T,
}

impl App<PersistenceStorage<BackendDB>> {
    pub async fn new(config: Config) -> Result<Self, Error> {
        let db = BackendDB::new(config.db.clone());
        let storage = PersistenceStorage::new(db);

        Ok(App {
            config,
            storage,
        })
    }
}
