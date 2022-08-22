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
use cosmoscout_models::{
    db::{BackendDB},
    models::{
        block::{NewBlock},
        chain::{NewChain},
        transaction::NewTransaction,
    }, storage::{StorageWriter, PersistenceStorage},
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
use cosmoscout_models::storage::StorageReader;

/// App is for fetching ABCI blocks, transactions and logs.
pub struct App<T:StorageWriter + StorageReader> {
    pub config: Config,
    pub storage: T,
    pub grpc_client: Arc<Mutex<ServiceClient<Channel>>>,
    pub tendermint_client: Arc<HttpClient>,
}

impl App<PersistenceStorage<BackendDB>> {
    pub async fn new(config: Config) -> Result<Self, Error> {
        let db = BackendDB::new(config.db.clone());
        let storage = PersistenceStorage::new(db);

        let grpc_client = ServiceClient::connect(config.fetcher.cosmos_grpc.clone())
            .await
            .map(Mutex::new)
            .map(Arc::new)?;

        let tendermint_client = HttpClient::new(config.fetcher.tendermint_rpc.as_str())
            .map(Arc::<HttpClient>::new)
            .map_err(|e| Error::from(e))?;

        Ok(App {
            config,
            storage,
            grpc_client,
            tendermint_client,
        })
    }

    pub async fn start(&self) -> Result<(), Error> {
        info!("fetcher app started to process");
        let mut retry_count = 0;
        let fetcher_config = &self.config.fetcher;

        if fetcher_config.start_block == 0 {
            return Err(Error::StartBlockMustBeGreaterThanZero);
        }

        // insert new chain if not exists
        self.insert_chain_config(&self.config.chain);

        let chain = self.storage.find_by_chain_id(self.config.chain.chain_id.clone())?;

        // connect to the tendermint rpc server
        let start_block = Height::from(fetcher_config.start_block);

        let mut current_block = self.get_start_block(
            start_block,
            chain.id,
            fetcher_config.try_resume_from_db,
        )?;
        info!("start to listen blocks from height `{}`", current_block);

        loop {
            match self.fetch_and_save_block(current_block, chain.id).await {
                Ok(_) => {
                    retry_count = 0;
                    current_block = current_block.increment();
                }
                Err(e) => {
                    if retry_count > 10 {
                        error!("unexpected error during fetching blockchain: {:?}", e);
                        panic!("teardown the fetcher");
                    }
                    sleep(Duration::from_millis(1000)).await;
                    retry_count += 1;
                }
            }
        }
    }

    async fn fetch_and_save_block(
        &self,
        block_height: Height,
        chain_id: i32,
    ) -> Result<bool, Error> {
        let block: block::Response = self.tendermint_client.block(block_height).await?;
        info!(
            "block fetched hash {:?}, block_number: {}",
            block.block.header.hash(),
            block_height
        );

        let block_results: block_results::Response =
            self.tendermint_client.block_results(block_height).await?;

        // fetch all transactions
        let future_fetch_txes = block
            .block
            .data
            .iter()
            .map(|tx| self.convert_txhash(tx))
            .map(|hash| {
                let t_client = self.grpc_client.clone();
                async move { t_client.lock().await.get_tx(GetTxRequest { hash }).await }
            });

        // one of action for fetching transaction could fail
        let fetch_txs_result: Vec<Result<tonic::Response<GetTxResponse>, tonic::Status>> =
            future::join_all(future_fetch_txes).await;

        let (ok, err): (Vec<_>, Vec<_>) = fetch_txs_result.into_iter().partition(|e| e.is_ok());

        if !err.is_empty() {
            return Err(Error::FetchingTransactionFailed);
        }

        let txes = ok
            .into_iter()
            .map(|x| x.unwrap())
            .collect::<Vec<tonic::Response<GetTxResponse>>>();

        let current_time = current_time();
        self.storage.within_transaction(|| {
            let mut new_block: NewBlock = NewBlockSchema::from(block.block).into();
            new_block.chain_id = chain_id;
            self.storage.insert_block(&new_block)?;

            let begin_block_events =
                extract_begin_block_events(&block_results, chain_id, &current_time);
            let end_block_events =
                extract_end_block_events(&block_results, chain_id, &current_time);

            for event in begin_block_events {
                self.storage.insert_event(&event)?;
            }

            for event in end_block_events {
                self.storage.insert_event(&event)?;
            }

            for tx in txes.into_iter() {
                let inner_tx = tx.get_ref();

                // construct new events
                let tx_events = extract_tx_events(&inner_tx, chain_id, &current_time);
                for event in tx_events {
                    self.storage.insert_event(&event)?;
                }

                // store transction information
                let mut new_tx: NewTransaction = NewTxSchema::from(inner_tx).into();
                new_tx.chain_id = chain_id;
                self.storage.insert_transaction(&new_tx)?;
            }

            Ok(true)
        }).map_err(|e| e.into())
    }

    /// resume start block from db
    fn get_start_block(
        &self,
        start_block: Height,
        chain_id: i32,
        try_resume_from_db: bool,
    ) -> Result<Height, Error> {
        // fetch latest block heights
        if try_resume_from_db {
            let latest_block_height = self.storage.latest_block_height(chain_id);

            match latest_block_height {
                Ok(height) => {
                    info!("fetcher resumed block height from db {}", height);
                    Ok(Height::from(height as u32).increment())
                }
                Err(_) => {
                    warn!("failed to fetch latest block height from db");
                    Ok(start_block)
                }
            }
        } else {
            Ok(start_block)
        }
    }

    /// convert block.data into transaction hash
    fn convert_txhash(&self, data: impl AsRef<[u8]>) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data);
        let tx_hash = hasher.finalize();
        format!("{:X}", tx_hash)
    }

    /// save chain config if it doesn't exists
    fn insert_chain_config(&self, chain_config: &ChainConfig) {
        let chain = self.storage.find_by_chain_id(chain_config.chain_id.clone());

        match chain {
            Ok(_) => debug!("chain already registered"),
            Err(e) => match e {
                cosmoscout_models::errors::Error::NotFound => {
                    let new_chain = &NewChain {
                        chain_id: chain_config.chain_id.clone(),
                        chain_name: chain_config.chain_name.clone(),
                        inserted_at: current_time(),
                    };
                    self.storage.insert_chain(&new_chain)
                        .unwrap_or_else(|e| panic!("failed to insert chain information: {:?}", e));
                }
                _ => panic!("unknown error during fetching chain infroation, {:?}", e),
            },
        }
    }
}
