use crate::{
    config::{ChainConfig, Config},
    utils::current_time, extension::{NewBlockSchema, NewTxSchema}, errors::FetchError,
};
use cosmoscout_models::{
    db::{BackendDB, Database},
    errors::DBModelError,
    models::{
        block::{Block, NewBlock},
        chain::{Chain, NewChain}, transaction::NewTransaction,
    },
};
use futures::future;
use log::{debug, error, info, warn};
use sha2::{Digest, Sha256};
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use tendermint::abci::transaction::Hash;
use tendermint::block::Height;
use tendermint_rpc::{Client, HttpClient};
use tokio::time::sleep;

/// FetcherApp is for fetching ABCI blocks, transactions and logs.
pub struct FetcherApp {
    pub config: Config,
    pub db: BackendDB,
}

impl FetcherApp {
    pub fn new(config: Config) -> Self {
        let mut db = BackendDB::new(config.db.clone());
        let connected = db.connect();
        if !connected {
            panic!("unable to connect to the database");
        }

        FetcherApp { config, db }
    }

    pub async fn start(&self) {
        info!("fetcher app started to process");
        let mut retry_count = 0;
        let fetcher_config = &self.config.fetcher;

        if fetcher_config.start_block == 0 {
            panic!("start block must be greater than 0");
        }

        // insert new chain if not exists
        self.insert_chain_config(&self.config.chain);
        let chain = Chain::find_by_chain_id(&self.db.conn().unwrap(), &self.config.chain.chain_id)
            .unwrap_or_else(|_| {
                panic!("failed to fetch chain information");
            });

        // connect to the tendermint rpc server
        let start_block = Height::from(fetcher_config.start_block);
        let client = HttpClient::new(fetcher_config.tendermint_rpc.as_str())
            .map(Arc::<HttpClient>::new)
            .unwrap_or_else(|_| {
                panic!(
                    "failed to connect to the tendermint rpc, endpoint: {}",
                    fetcher_config.tendermint_rpc
                );
            });
       
        let mut current_block = self.get_start_block(start_block, fetcher_config.chain_id.as_str(), fetcher_config.try_resume_from_db);
        info!("start to listen blocks from height `{}`", current_block);

        loop {
            let client = client.clone();
            match self.fetch_and_save_block(client, current_block, chain.id).await {
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
        client: Arc<HttpClient>,
        block_height: Height,
        chain_id: i32,
    ) -> Result<bool, FetchError> {
        let block = client.block(block_height).await?;
        info!(
            "block fetched hash {:?}, block_number: {}",
            block.block.header.hash(),
            block_height
        );

        let block_results = client.block_results(block_height).await?;

        // fetch all transactions
        let future_fetch_txes = block
            .block
            .data
            .iter()
            .map(|tx| self.convert_txhash(tx))
            .map(|hash| {
                let hash_wrapped = Hash::from_str(hash.as_str()).unwrap();
                let t_client = client.clone();

                async move { t_client.tx(hash_wrapped, false).await }
            });

        // one of action for fetching transaction could fail
        let fetch_txs_result = future::join_all(future_fetch_txes).await;
        let (ok, err):(Vec<_>, Vec<_>) = fetch_txs_result.into_iter()
            .partition(|e| e.is_ok());
        
        if err.len() > 0 {
            return Err(FetchError::FetchingTransactionFailed);
        }

        let txes = ok.into_iter()
            .map(|x| x.unwrap())
            .collect::<Vec<_>>();

        let conn = self.db.conn()
            .unwrap_or_else(|| panic!("cannot get database connection, we may losse it?"));

        conn.build_transaction()
            .repeatable_read()
            .run::<bool, DBModelError, _>(|| {
                let mut new_block:NewBlock = NewBlockSchema::from(block.block).into();
                new_block.chain_id = chain_id;
                NewBlock::insert(&conn, &new_block)?;

                for tx in txes.into_iter() {
                    let mut new_tx: NewTransaction = NewTxSchema::from(tx).into();
                    new_tx.chain_id = chain_id;
                    NewTransaction::insert(&conn, &new_tx)?;
                }

                Ok(true)
            })
            .map_err(|e| e.into())
    }

    /// resume start block from db
    fn get_start_block(&self, start_block: Height, chain_id: &str, try_resume_from_db: bool) -> Height {
         // fetch latest block heights
         if try_resume_from_db {
             let chain =
                 Chain::find_by_chain_id(&self.db.conn().unwrap(), chain_id)
                     .unwrap_or_else(|_| panic!("failed to get chain information from db"));
 
             let latest_block_height =
                 Block::latest_block_height(&self.db.conn().unwrap(), chain.id);
 
             match latest_block_height {
                 Ok(height) => {
                     info!("fetcher resumed block height from db {}", height);
                     Height::from(height as u32).increment()
                 }
                 Err(_) => {
                    warn!("failed to fetch latest block height from db");
                    start_block
                 }
             }
         } else {
            start_block
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
        let db = &self.db;

        let chain = Chain::find_by_chain_id(&db.conn().unwrap(), chain_config.chain_id.as_str());
        match chain {
            Ok(_) => debug!("chain already registered"),
            Err(e) => match e {
                DBModelError::NotFound => {
                    NewChain::insert(
                        &NewChain {
                            chain_id: chain_config.chain_id.clone(),
                            chain_name: chain_config.chain_name.clone(),
                            inserted_at: current_time(),
                        },
                        &db.conn().unwrap(),
                    )
                    .unwrap_or_else(|e| panic!("failed to insert chain information: {:?}", e));
                }
                _ => panic!("unknown error during fetching chain infroation, {:?}", e),
            },
        }
    }
}
