use std::{time::Duration};
use log::{info, error};
use sha2::{Sha256, Digest};
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

        let start_block = Height::from(self.config.start_block);
        let client = match HttpClient::new(self.config.tendermint_rpc.as_str()) {
            Ok(c) => c,
            Err(e) => {
                panic!("failed to connect to the tendermint rpc, endpoint: {}, err: {}", self.config.tendermint_rpc, e);
            }
        };

        let mut current_block = start_block.clone();
        info!("start to listen blocks from `{}` height", current_block);

        loop {
            let result = client.block(current_block).await;
            match result {
                Ok(block) => {
                    info!("block fetched hash: {:?}, block_number: {}", block.block.header.hash(), current_block);
                    current_block = current_block.increment();

                    for tx in block.block.data.iter() {
                        let hash = self.data_to_hash(tx);
                        info!("tx_hash found: {}", hash);
                    }
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