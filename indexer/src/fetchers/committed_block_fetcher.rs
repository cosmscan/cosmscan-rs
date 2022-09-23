use crate::errors::Error;
use crate::messages::MsgCommittedBlock;

use cosmos_client::response;

use log::info;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;

use std::sync::Arc;
use std::time::Duration;

/// CommittedBlockFetcher fetches blocks, transactions, and events from Tendermint RPC and Cosmos REST API
pub struct CommittedBlockFetcher {
    pub client: Arc<Mutex<cosmos_client::client::Client>>,
}

impl CommittedBlockFetcher {
    /// creates a new CommittedBlockFetcher instance.
    pub async fn new(client: Arc<Mutex<cosmos_client::client::Client>>) -> Result<Self, Error> {
        Ok(CommittedBlockFetcher { client })
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
            let client = self.client.clone();
            async move { client.lock().await.get_transaction(tx_hash.clone()).await }
        });

        for result in futures::future::join_all(future_transactions).await {
            match result {
                Ok((tx, _events)) => {
                    transactions.push(tx);
                    events.extend(_events);
                }
                Err(e) => {
                    return Err(Error::Other(e.to_string()));
                }
            }
        }

        // wrap transactions with messages
        let tx_with_messages = transactions.into_iter().map(|tx| {
            let client = self.client.clone();

            async move {
                let messages = client
                    .lock()
                    .await
                    .get_tx_messages(tx.transaction_hash.clone())
                    .await?;

                let mut _tx = tx.clone();
                _tx.messages.extend(messages);
                Ok::<response::Transaction, Error>(_tx)
            }
        });

        let mut transactions: Vec<response::Transaction> = vec![];
        for result in futures::future::join_all(tx_with_messages).await {
            match result {
                Ok(tx) => {
                    transactions.push(tx);
                }
                Err(e) => {
                    return Err(Error::Other(e.to_string()));
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

pub struct CommittedBlockWorker;
impl CommittedBlockWorker {
    pub fn spawn(
        fetcher: CommittedBlockFetcher,
        sender: tokio::sync::mpsc::Sender<MsgCommittedBlock>,
        start_block: i64,
    ) -> JoinHandle<()> {
        tokio::spawn(async move {
            let mut current_block = start_block;

            loop {
                match fetcher.committed_block_at(current_block).await {
                    Ok(committed_block) => {
                        let sender = sender.clone();
                        info!(
                            "commit block | block_number: {}, hash: {}",
                            current_block,
                            committed_block.block.block_hash.clone(),
                        );
                        sender.send(committed_block).await.unwrap();
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
                                    log::error!("error: {}", e);
                                    panic!("committed block worker has been failed unexpectedly");
                                }
                            }
                            _ => {
                                panic!("committed block worker has been failed unexpectedly");
                            }
                        }
                    }
                }
            }
        })
    }
}
