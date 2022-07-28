use std::{time::Duration};
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use futures::future;
use log::{info, error};
use sha2::{Sha256, Digest};
use tendermint::abci::transaction::Hash;
use tendermint::block::Height;
use tendermint_rpc::{Client, HttpClient};
use tokio::time::sleep;
use crate::config::FetcherConfig;

/// FetcherApp is for fetching ABCI blocks, transactions and logs.
pub struct FetcherApp {
    pub config: FetcherConfig,
}

impl FetcherApp {
    pub fn new(config: FetcherConfig) -> Self {
        FetcherApp{
            config,
        }
    }

    pub async fn start(&self) {
        info!("fetcher app started to process");

        if self.config.start_block == 0 {
            panic!("start block must be greater than 0");
        }

        // connect to the tendermint rpc server
        let start_block = Height::from(self.config.start_block);
        let client = HttpClient::new(self.config.tendermint_rpc.as_str())
            .map(|c| Arc::new(c))
            .expect(format!("failed to connect to the tendermint rpc, endpoint: {}", self.config.tendermint_rpc).as_str());

        // start from current block
        let mut current_block = start_block.clone();
        info!("start to listen blocks from `{}` height", current_block);

        loop {
            let client = client.clone();
            let result = client.block(current_block).await;
            match result {
                Ok(block) => {
                    info!("block fetched hash: {:?}, block_number: {}", block.block.header.hash(), current_block);

                    let block_results = client.block_results(current_block).await.unwrap();
                    info!("block_result: {:?}, block_number: {}", block_results, current_block);

                    let future_fetch_txes = block.block.data.iter()
                        .map(|tx| self.data_to_hash(tx))
                        .map(|hash| {
                            let hash_wrapped = Hash::from_str(hash.as_str()).unwrap();
                            let t_client = client.clone();

                            async move { t_client.tx(hash_wrapped, false).await.unwrap() }
                        });

                    let results = future::join_all(future_fetch_txes).await;

                    for tx in results.iter() {
                        info!("transaction received: {:?}", tx);
                    }

                    current_block = current_block.increment();
                },
                Err(_) => {
                    sleep(Duration::from_millis(500)).await;
                },
            }
        }
    }

    /// convert block.data into transaction hash
    fn data_to_hash(&self, data: impl AsRef<[u8]>) -> String{
        let mut hasher = Sha256::new();
        hasher.update(data);
        let tx_hash = hasher.finalize();
        format!("{:X}", tx_hash)
    }
}