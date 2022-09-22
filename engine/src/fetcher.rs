use crate::config::FetcherConfig;
use crate::errors::Error;
use crate::messages::MsgCommittedBlock;

use cosmos_client::response;

use log::info;
use std::collections::HashMap;

use std::sync::Arc;
use std::time::Duration;

use tokio::sync::Mutex;

/// Fetcher fetches blocks, transactions, and events from Tendermint RPC and Cosmos REST API
pub struct Fetcher {
    pub client: Arc<Mutex<cosmos_client::client::Client>>,
    pub config: FetcherConfig,
    pub sender: tokio::sync::mpsc::Sender<MsgCommittedBlock>,
    pub start_block: i64,
}

impl Fetcher {
    /// creates a new Fetcher instance.
    pub async fn new(
        config: FetcherConfig,
        sender: tokio::sync::mpsc::Sender<MsgCommittedBlock>,
        start_block: i64,
    ) -> Result<Self, Error> {
        let client = cosmos_client::client::Client::new(cosmos_client::client::ClientConfig {
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

    pub async fn run(&self) -> Result<(), Error> {
        // start_block must be greater than zero int.
        if self.start_block <= 0 {
            return Err(Error::StartBlockMustBeGreaterThanZero);
        }

        let mut current_block = self.start_block;
        let journal: Arc<Mutex<HashMap<i64, MsgCommittedBlock>>> =
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
                    match e {
                        Error::CosmosClientError(cosmos_client::errors::Error::RPCError(
                            tendermint_rpc::Error(
                                tendermint_rpc::error::ErrorDetail::Response(ref resp),
                                _,
                            ),
                        )) => {
                            if resp.source.code() == tendermint_rpc::Code::InternalError {
                                // wait for new block
                                // this error occurred when the block given as parameter is not yet proposed by the validator
                                // Caused by:
                                //      Internal error: height 129 must be less than or equal to the current blockchain height 128 (code: -32603)
                                tokio::time::sleep(Duration::from_millis(2000)).await;
                            } else {
                                return Err(e);
                            }
                        }
                        _ => {
                            return Err(e);
                        }
                    }
                }
            }
        }
    }

    async fn committed_block_at(&self, block_height: i64) -> Result<MsgCommittedBlock, Error> {
        // get block info from given height
        let (block, tx_hashes) = self.client.lock().await.get_block(block_height).await?;
        info!(
            "found new block | block_number:{}, hash: {}",
            block.height,
            block.block_hash.clone(),
        );

        // get block result from given height
        let block_result = self
            .client
            .lock()
            .await
            .get_block_result(block_height)
            .await?;

        // fetch trasnactions
        let mut transactions: Vec<response::Transaction> = vec![];
        let mut events: Vec<response::Event> = vec![];
        events.extend(block_result.begin_block_events);
        events.extend(block_result.end_block_events);

        let future_transactions = tx_hashes.into_iter().map(|tx_hash| {
            tokio::spawn({
                let client = self.client.clone();
                async move { client.lock().await.get_transaction(tx_hash.clone()).await }
            })
        });

        for result in futures::future::join_all(future_transactions).await {
            match result {
                Ok(Ok((tx, _events))) => {
                    transactions.push(tx);
                    events.extend(_events);
                }
                Ok(Err(err)) => {
                    return Err(Error::CosmosClientError(err));
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
                Ok::<response::Transaction, Error>(_tx)
            })
        });

        let mut transactions: Vec<response::Transaction> = vec![];
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

        Ok(MsgCommittedBlock {
            block: block,
            txs: transactions,
            events,
        })
    }
}
