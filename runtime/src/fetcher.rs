use log::{error, info};
use tendermint::block::Height;
use tendermint_rpc::{Client, HttpClient, Error};
use tokio::runtime::Runtime;
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

    pub fn start(&self) {
        info!("fetcher app started to process");

        let start_block = Height::from(self.config.start_block);
        let client = match HttpClient::new(self.config.tendermint_rpc.as_str()) {
            Ok(c) => c,
            Err(e) => {
                panic!("failed to connect to the tendermint rpc, endpoint: {}, err: {}", self.config.tendermint_rpc, e);
            }
        };
    }
}