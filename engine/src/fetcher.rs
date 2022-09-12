use crate::client::{Client, ClientConfig};
use crate::config::FetcherConfig;
use crate::errors::Error;
use crate::rawdata::{RawBlock, RawEvent, RawTx};
use crate::utils::bytes_to_tx_hash;

use cosmoscout_models::models::event::{
    TX_TYPE_BEGIN_BLOCK, TX_TYPE_END_BLOCK, TX_TYPE_TRANSACTION,
};
use log::info;
use std::collections::HashMap;
use std::str::from_utf8;
use std::sync::Arc;
use std::time::Duration;
use tendermint::abci;
use tokio::sync::mpsc::channel;
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
        let result_channel = self.sender.clone();

        let _journal = journal.clone();
        tokio::spawn(async move {
            // send journal data to result channel
            loop {
                let mut journal = _journal.lock().await;
                if let Some(committed_block) = journal.get(&checkpoint_block) {
                    result_channel.send(committed_block.clone()).await.unwrap();
                    journal.remove(&checkpoint_block);
                    checkpoint_block += 1;
                }

                tokio::time::sleep(Duration::from_millis(200)).await;
            }
        });

        loop {
            // get block info from given height
            let block = self.client.lock().await.get_block(current_block).await?;
            info!(
                "fetcher listens block_number:{}, hash: {}",
                block.header.height,
                block.header.hash(),
            );

            // get block result from given height
            let block_result = self
                .client
                .clone()
                .lock()
                .await
                .get_block_result(current_block)
                .await?;

            // fetch trasnactions
            let mut transactions: Vec<RawTx> = vec![];
            let mut events: Vec<RawEvent> = vec![];

            let (tx_sender, mut tx_receiver) = channel::<RawTx>(100);
            let (evt_sender, mut event_receiver) = channel::<RawEvent>(100);

            // check block data is empty list
            if block.data.iter().len() == 0 {
                drop(tx_sender);
                drop(evt_sender);
            } else {
                let handles = block.data.iter().map(|data| {
                    let tx_hash = bytes_to_tx_hash(data);
                    let client = self.client.clone();

                    let tx_sender = tx_sender.clone();
                    let evt_sender = evt_sender.clone();

                    tokio::spawn(async move {
                        let tx = client
                            .lock()
                            .await
                            .get_transaction(tx_hash.clone())
                            .await
                            .unwrap();
                        let mut raw_tx = RawTx::from(&tx);

                        let messages = client
                            .lock()
                            .await
                            .get_tx_messages(tx_hash.clone())
                            .await
                            .unwrap();
                        raw_tx.messages.extend(messages);

                        if let Some(tx_resp) = tx.tx_response {
                            for evt in tx_resp.events.iter() {
                                for attr in evt.attributes.iter() {
                                    let raw_event = RawEvent {
                                        tx_type: TX_TYPE_TRANSACTION,
                                        tx_hash: Some(tx_hash.clone()),
                                        event_type: evt.r#type.clone(),
                                        event_key: from_utf8(&attr.key).unwrap().to_string(),
                                        event_value: from_utf8(&attr.value).unwrap().to_string(),
                                        indexed: attr.index,
                                    };
                                    evt_sender.send(raw_event).await.unwrap();
                                }
                            }
                        }

                        tx_sender.send(raw_tx).await.unwrap();
                    })
                });

                for h in handles {
                    h.await.unwrap();
                }

                drop(tx_sender);
                drop(evt_sender);
            }
            loop {
                tokio::select! {
                    Some(tx) = tx_receiver.recv() => {
                        transactions.push(tx);
                    }
                    Some(evt) = event_receiver.recv() => {
                        events.push(evt);
                    }
                    else => {
                        break;
                    }
                }
            }

            // expand block result to get begin block & end block events
            if let Some(begin_block_events) = block_result.begin_block_events {
                let begin_blocks =
                    Self::convert_block_events(begin_block_events, TX_TYPE_BEGIN_BLOCK);
                events.extend(begin_blocks);
            }

            if let Some(end_block_events) = block_result.end_block_events {
                let end_blocks = Self::convert_block_events(end_block_events, TX_TYPE_END_BLOCK);
                events.extend(end_blocks);
            }

            journal.clone().lock().await.insert(
                current_block,
                MsgCommittedBlock {
                    block: RawBlock::from(block),
                    txs: transactions,
                    events: events,
                },
            );

            current_block += 1;
        }
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
