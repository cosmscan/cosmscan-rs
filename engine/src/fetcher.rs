use crate::client::{Client, ClientConfig};
use crate::config::FetcherConfig;
use crate::errors::Error;
use crate::rawdata::{RawBlock, RawEvent, RawTx};
use crate::utils::bytes_to_tx_hash;

use cosmos_sdk_proto::cosmos::distribution::v1beta1::msg_server::Msg;
use cosmoscout_models::models::event::{
    TX_TYPE_BEGIN_BLOCK, TX_TYPE_END_BLOCK, TX_TYPE_TRANSACTION,
};
use log::{error, info};
use std::collections::HashMap;
use std::str::from_utf8;
use std::sync::Arc;
use std::time::Duration;
use tendermint::abci;
use tokio::sync::Mutex;

/// MsgCommittedBlock is a message which indicates committed block.
/// It's intended to be sent to the sender channel of [`Fetcher`].
#[derive(Debug, Clone, PartialEq)]
pub struct MsgCommittedBlock {
    pub block: RawBlock,
    pub txs: Vec<RawTx>,
    pub events: Vec<RawEvent>,
}

/// Fetcher fetches blocks, transactions, and events from Tendermint RPC and Cosmos REST API
pub struct Fetcher {
    pub client: Arc<Mutex<Client>>,
    pub config: FetcherConfig,
    pub sender: tokio::sync::mpsc::Sender<MsgCommittedBlock>,
    pub start_block: u64,
}

impl Fetcher {
    /// creates a new Fetcher instance.
    pub async fn new(
        config: FetcherConfig,
        sender: tokio::sync::mpsc::Sender<MsgCommittedBlock>,
        start_block: u64,
    ) -> Result<Self, Error> {
        let client = Client::new(ClientConfig {
            tendermint_rpc_endpoint: config.tendermint_rpc_endpoint.clone(),
            grpc_endpoint: config.grpc_endpoint.clone(),
            rest_api_endpoint: config.rest_api_endpoint.clone(),
        })
        .await
        .map(Mutex::new)
        .map(Arc::new)?;

        Ok(Fetcher {
            client,
            config,
            sender,
            start_block,
        })
    }

    pub async fn run_fetch_loop(&self) -> Result<(), Error> {
        // start_block must be greater than zero int.
        if self.start_block <= 0 {
            return Err(Error::StartBlockMustBeGreaterThanZero);
        }

        let mut current_block = self.start_block;
        let journal: Arc<Mutex<HashMap<u64, MsgCommittedBlock>>> =
            Arc::new(Mutex::new(HashMap::new()));
        let mut checkpoint_block = current_block;

        tokio::spawn({
            let _journal = journal.clone();
            let sender = self.sender.clone();
            async move {
                // send journal data to result channel
                loop {
                    let mut journal = _journal.lock().await;
                    if let Some(committed_block) = journal.get(&checkpoint_block) {
                        sender.send(committed_block.clone()).await.unwrap();
                        journal.remove(&checkpoint_block);
                        checkpoint_block += 1;
                    }

                    tokio::time::sleep(Duration::from_millis(200)).await;
                }
            }
        });

        loop {
            match self.committed_block_at(current_block).await {
                Ok(committed_block) => {
                    // insert to the journal and print size of journal
                    let mut journal = journal.lock().await;
                    info!(
                        "commit block | block_number: {}, hash: {}",
                        current_block,
                        committed_block.block.block_hash.clone(),
                    );
                    info!("journal size: {}", journal.len());
                    journal.insert(current_block, committed_block);
                    current_block += 1;
                }
                Err(e) => {
                    error!(
                        "failed to get committed block at given height: {}, err: {:?}",
                        current_block, e
                    );
                    tokio::time::sleep(Duration::from_millis(1000)).await;
                }
            }
        }
    }

    async fn committed_block_at(&self, block_height: u64) -> Result<MsgCommittedBlock, Error> {
        // get block info from given height
        let block = self.client.lock().await.get_block(block_height).await?;
        info!(
            "found new block | block_number:{}, hash: {}",
            block.header.height,
            block.header.hash(),
        );

        // get block result from given height
        let block_result = self
            .client
            .lock()
            .await
            .get_block_result(block_height)
            .await?;

        // fetch trasnactions
        let mut transactions: Vec<RawTx> = vec![];
        let mut events: Vec<RawEvent> = vec![];

        let future_transactions = block.data.iter().map(bytes_to_tx_hash).map(|tx_hash| {
            tokio::spawn({
                let client = self.client.clone();
                async move { client.lock().await.get_transaction(tx_hash).await }
            })
        });

        for result in futures::future::join_all(future_transactions).await {
            match result {
                Ok(Ok(tx)) => {
                    if let Some(tx_resp) = &tx.tx_response {
                        for evt in tx_resp.events.iter() {
                            for attr in evt.attributes.iter() {
                                let raw_event = RawEvent {
                                    tx_type: TX_TYPE_TRANSACTION,
                                    tx_hash: Some(tx_resp.txhash.clone()),
                                    event_type: evt.r#type.clone(),
                                    event_key: from_utf8(&attr.key)?.to_string(),
                                    event_value: from_utf8(&attr.value)?.to_string(),
                                    indexed: attr.index,
                                };
                                events.push(raw_event);
                            }
                        }
                    }

                    transactions.push(RawTx::from(&tx));
                }
                Ok(Err(err)) => {
                    return Err(err);
                }
                Err(e) => {
                    return Err(Error::Other(e.to_string()));
                }
            }
        }

        // wrap transactions with messages
        let tx_with_messages = transactions.into_iter().map(|tx| {
            let client = self.client.clone();
            tokio::spawn(async move {
                let messages = client
                    .lock()
                    .await
                    .get_tx_messages(tx.transaction_hash.clone())
                    .await?;
                let mut _tx = tx.clone();
                _tx.messages.extend(messages);
                Ok::<RawTx, Error>(_tx)
            })
        });

        let mut transactions: Vec<RawTx> = vec![];
        for result in futures::future::join_all(tx_with_messages).await {
            match result {
                Err(e) => {
                    return Err(Error::Other(e.to_string()));
                }
                Ok(Err(e)) => {
                    return Err(e);
                }
                Ok(Ok(tx)) => {
                    transactions.push(tx);
                }
            }
        }

        // expand block result to get begin block & end block events
        if let Some(begin_block_events) = block_result.begin_block_events {
            let begin_blocks = Self::convert_block_events(begin_block_events, TX_TYPE_BEGIN_BLOCK);
            events.extend(begin_blocks);
        }

        if let Some(end_block_events) = block_result.end_block_events {
            let end_blocks = Self::convert_block_events(end_block_events, TX_TYPE_END_BLOCK);
            events.extend(end_blocks);
        }

        Ok(MsgCommittedBlock {
            block: RawBlock::from(block),
            txs: transactions,
            events,
        })
    }

    fn convert_block_events(events: Vec<abci::Event>, tx_type: i16) -> Vec<RawEvent> {
        events
            .iter()
            .map(|evt| {
                evt.attributes.iter().map(|attr| RawEvent {
                    tx_type: tx_type,
                    tx_hash: None,
                    event_type: evt.type_str.clone(),
                    event_key: attr.key.to_string(),
                    event_value: attr.value.to_string(),
                    indexed: false,
                })
            })
            .flatten()
            .collect::<Vec<RawEvent>>()
    }
}
