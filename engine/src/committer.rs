use std::{collections::HashMap, sync::Arc};

use cosmos_client::response::EventType;
use cosmscan_models::{
    config::DBConfig,
    db::BackendDB,
    models::{
        block::NewBlock,
        chain::{Chain},
        event::{NewEvent, TX_TYPE_BEGIN_BLOCK, TX_TYPE_END_BLOCK, TX_TYPE_TRANSACTION},
        message::NewMessage,
        transaction::NewTransaction,
    },
    storage::{PersistenceStorage, StorageWriter},
};
use log::{error, info};
use tokio::{
    sync::{mpsc, Mutex},
    task::JoinHandle,
};

use crate::{current_time, errors::Error, messages::MsgCommittedBlock};

pub struct Committer {
    storage: PersistenceStorage<BackendDB>,
    chain_info: Chain,
    checkpoint_block: i64,
    committed_blocks: Arc<Mutex<HashMap<i64, MsgCommittedBlock>>>,
    subscribe_rx: mpsc::UnboundedReceiver<MsgCommittedBlock>,
    receive_loop: JoinHandle<()>,
}

impl Committer {
    /// Creates a new committer instance
    pub fn new(
        dbconfig: DBConfig,
        chain_info: Chain,
        committed_block_c: mpsc::Receiver<MsgCommittedBlock>,
        checkpoint_block: i64,
    ) -> Committer {
        let backend_db = BackendDB::new(dbconfig);
        let storage = PersistenceStorage::new(backend_db);
        let committed_blocks: Arc<Mutex<HashMap<i64, MsgCommittedBlock>>> =
            Arc::new(Mutex::new(HashMap::new()));
        let (subscribe_tx, subscribe_rx) = mpsc::unbounded_channel();

        Committer {
            storage,
            chain_info,
            checkpoint_block,
            committed_blocks,
            subscribe_rx,
            receive_loop: tokio::spawn(Committer::run_receive_loop(
                committed_block_c,
                subscribe_tx,
            )),
        }
    }

    /// Run committing block loop
    /// It receives committed blocks through channel and commits them to storage.
    pub async fn start(&mut self) -> Result<(), Error> {
        while let Some(msg) = self.subscribe_rx.recv().await {
            let given_height = msg.block.height;
            if given_height == self.checkpoint_block {
                match self.commit_to_storage(&self.chain_info, msg) {
                    Ok(_) => {
                        info!("committed new block at {}", given_height);
                        self.checkpoint_block += 1;
                    }
                    Err(err) => {
                        error!("failed to commit data to the storage: {:?}", err);
                        return Err(err);
                    }
                }
            } else {
                self.committed_blocks.lock().await.insert(given_height, msg);
            }
        }

        Ok(())
    }

    pub fn commit_to_storage(&self, chain: &Chain, msg: MsgCommittedBlock) -> Result<bool, Error> {
        self.storage
            .within_transaction(|| {
                let block = msg.block;
                let txs = msg.txs;

                // insert block
                self.storage.insert_block(&NewBlock {
                    chain_id: chain.id,
                    height: block.height,
                    block_hash: block.block_hash,
                    prev_hash: block.prev_hash,
                    proposer_address: block.proposer_address,
                    last_commit_hash: block.last_commit_hash,
                    data_hash: block.data_hash,
                    validators_hash: block.validators_hash,
                    next_validators_hash: block.next_validators_hash,
                    consensus_hash: block.consensus_hash,
                    app_hash: block.app_hash,
                    last_result_hash: block.last_result_hash,
                    evidence_hash: block.evidence_hash,
                    block_time: block.block_time,
                    inserted_at: current_time(),
                })?;

                // insert transactions
                for tx in txs {
                    let tx_id = self.storage.insert_transaction(&NewTransaction {
                        chain_id: chain.id,
                        height: tx.height,
                        transaction_hash: tx.transaction_hash,
                        code: tx.code,
                        code_space: tx.code_space,
                        tx_data: tx.tx_data,
                        raw_log: tx.raw_log,
                        info: tx.info,
                        memo: tx.memo,
                        gas_wanted: tx.gas_wanted,
                        gas_used: tx.gas_used,
                        tx_timestamp: tx.tx_timestamp,
                        inserted_at: current_time(),
                    })?;

                    for (seq, msg) in tx.messages.iter().enumerate() {
                        self.storage.insert_message(&NewMessage {
                            transaction_id: tx_id as i32,
                            seq: seq as i32,
                            rawdata: serde_json::Value::from(msg.clone()),
                            inserted_at: current_time(),
                        })?;
                    }
                }

                // insert events
                for event in msg.events {
                    let _type = match event.tx_type {
                        EventType::BeginBlock => TX_TYPE_BEGIN_BLOCK,
                        EventType::EndBlock => TX_TYPE_END_BLOCK,
                        EventType::Transaction => TX_TYPE_TRANSACTION,
                    };

                    self.storage.insert_event(&NewEvent {
                        chain_id: chain.id,
                        tx_type: _type,
                        tx_hash: event.tx_hash,
                        block_height: event.block_height,
                        event_type: event.event_type,
                        event_key: event.event_key,
                        event_value: event.event_value,
                        indexed: event.indexed,
                        inserted_at: current_time(),
                    })?;
                }

                // insert messages

                Ok(true)
            })
            .map_err(|e| e.into())
    }

    pub async fn run_receive_loop(
        mut channel: mpsc::Receiver<MsgCommittedBlock>,
        subscribe_tx: mpsc::UnboundedSender<MsgCommittedBlock>,
    ) -> () {
        while let Some(msg) = channel.recv().await {
            subscribe_tx.send(msg).unwrap();
        }
    }

    pub async fn shudown(self) {
        self.receive_loop.abort();
        self.receive_loop.await.unwrap();
    }
}
