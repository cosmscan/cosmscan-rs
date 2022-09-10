use crate::config::FetcherConfig;
use crate::convert::{NewBlockSchema, NewTxSchema};
use crate::errors::Error;
use crate::utils;
use cosmoscout_models::models::chain::Chain;
use cosmoscout_models::models::event::NewEvent;
use log::info;
use std::collections::HashMap;
use std::sync::Arc;
use tendermint::abci::transaction::Hash;
use tendermint_rpc::endpoint::block_results;
use tendermint_rpc::Client;

/// MsgCommittedBlock is a message which indicates committed block.
/// It's intended to be sent to the sender channel of [`Fetcher`].
pub struct MsgCommittedBlock {
    pub block: NewBlockSchema,
    pub txs: Vec<NewTxSchema>,
    pub events: Vec<NewEvent>,
}

/// Fetcher fetches blocks, transactions, and events from Tendermint RPC and Cosmos REST API
pub struct Fetcher {
    pub journal: HashMap<u64, MsgCommittedBlock>,
    pub config: FetcherConfig,
    pub chain_info: Chain,
    pub tendermint_client: Arc<tendermint_rpc::HttpClient>,
    pub sender: tokio::sync::mpsc::Sender<MsgCommittedBlock>,
    pub start_block: u64,
}

impl Fetcher {
    /// creates a new Fetcher instance.
    pub fn new(
        config: FetcherConfig,
        chain_info: Chain,
        sender: tokio::sync::mpsc::Sender<MsgCommittedBlock>,
        start_block: u64,
    ) -> Result<Self, Error> {
        let journal: HashMap<u64, MsgCommittedBlock> = HashMap::new();

        let tendermint_client = tendermint_rpc::HttpClient::new(config.tendermint_rpc.as_str())
            .map(Arc::<tendermint_rpc::HttpClient>::new)
            .map_err(|e| Error::from(e))?;

        Ok(Fetcher {
            journal,
            config,
            chain_info,
            tendermint_client,
            sender,
            start_block,
        })
    }

    pub async fn run(&mut self) -> Result<(), Error> {
        // validate start_block is greater than 0
        if self.start_block <= 0 {
            return Err(Error::StartBlockMustBeGreaterThanZero);
        }

        let mut current_block = self.start_block;
        loop {
            // get block info from given height
            let block_height = tendermint::block::Height::from(current_block as u32);
            let resp = self.tendermint_client.block(block_height).await?;
            info!(
                "fetcher listens block_number:{}, hash: {}",
                resp.block.header.height,
                resp.block.header.hash()
            );

            // get block result from given height
            let block_ret_resp: block_results::Response =
                self.tendermint_client.block_results(block_height).await?;

            self.journal.insert(
                current_block,
                MsgCommittedBlock {
                    block: NewBlockSchema::from(resp.block),
                    txs: vec![],
                    events: vec![],
                },
            );

            current_block += 1;
        }

        Ok(())
    }
}
