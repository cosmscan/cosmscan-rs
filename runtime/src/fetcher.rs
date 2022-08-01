use crate::config::FetcherConfig;
use futures::future;
use log::{error, info};
use sha2::{Digest, Sha256};
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tendermint::abci::transaction::Hash;
use tendermint::block::Height;
use tendermint_rpc::{Client, HttpClient};
use tokio::time::sleep;

/// FetcherApp is for fetching ABCI blocks, transactions and logs.
pub struct FetcherApp {
    pub config: FetcherConfig,
}

impl FetcherApp {
    pub fn new(config: FetcherConfig) -> Self {
        FetcherApp { config }
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
            .expect(
                format!(
                    "failed to connect to the tendermint rpc, endpoint: {}",
                    self.config.tendermint_rpc
                )
                .as_str(),
            );

        // start from current block
        let mut current_block = start_block.clone();
        info!("start to listen blocks from `{}` height", current_block);

        loop {
            let client = client.clone();
            match self.fetch_and_save_block(client, current_block).await {
                Ok(_) => {
                    current_block = current_block.increment();
                }
                Err(e) => {
                    error!("unexpected error during fetching blockchain: {:?}", e);
                    sleep(Duration::from_millis(200)).await;
                }
            }
        }
    }

    async fn fetch_and_save_block(
        &self,
        client: Arc<HttpClient>,
        block_height: Height,
    ) -> Result<(), Box<dyn std::error::Error>> {
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

        let fetch_txs_result = future::join_all(future_fetch_txes).await;

        info!(
            "block_result: {:?}, fetch_tx_result: {:?}",
            block_results, fetch_txs_result
        );

        // if one of transaction failed, then it should return error
        let has_error = fetch_txs_result
            .iter()
            .filter_map(|x| match x {
                Ok(_) => None,
                Err(err) => Some(err),
            })
            .collect::<Vec<_>>();

        if has_error.len() > 0 {
            return Err(has_error[0].clone().into());
        }

        Ok(())
    }

    /// convert block.data into transaction hash
    fn convert_txhash(&self, data: impl AsRef<[u8]>) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data);
        let tx_hash = hasher.finalize();
        format!("{:X}", tx_hash)
    }
}
