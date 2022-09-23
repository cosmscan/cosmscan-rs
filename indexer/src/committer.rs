use std::str::FromStr;

use cosmos_client::response::EventType;
use cosmscan_models::{
    config::DBConfig,
    db::BackendDB,
    models::{
        block::NewBlock,
        chain::Chain,
        event::{NewEvent, TX_TYPE_BEGIN_BLOCK, TX_TYPE_END_BLOCK, TX_TYPE_TRANSACTION},
        message::NewMessage,
        transaction::NewTransaction,
    },
    storage::{PersistenceStorage, StorageWriter},
};

use crate::{current_time, errors::Error, messages::MsgCommittedBlock};

pub struct Committer {
    storage: PersistenceStorage<BackendDB>,
    chain_info: Chain,
}

impl Committer {
    /// Creates a new committer instance
    pub fn new(dbconfig: DBConfig, chain_info: Chain) -> Committer {
        let backend_db = BackendDB::new(dbconfig);
        let storage = PersistenceStorage::new(backend_db);

        Committer {
            storage,
            chain_info,
        }
    }

    pub fn commit_block(&self, msg: MsgCommittedBlock) -> Result<bool, Error> {
        self.storage
            .within_transaction(|| {
                let block = msg.block;
                let txs = msg.txs;

                // insert block
                self.storage.insert_block(&NewBlock {
                    chain_id: self.chain_info.id,
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
                    let new_tx = self.storage.insert_transaction(&NewTransaction {
                        chain_id: self.chain_info.id,
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
                            transaction_id: new_tx.id,
                            seq: seq as i32,
                            rawdata: serde_json::Value::from_str(msg.clone().as_str())
                                .expect("cannot unmarshal cosmos message to json"),
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
                        chain_id: self.chain_info.id,
                        tx_type: _type,
                        tx_hash: event.tx_hash,
                        block_height: event.block_height,
                        event_seq: event.event_seq,
                        event_type: event.event_type,
                        event_key: event.event_key,
                        event_value: event.event_value,
                        indexed: event.indexed,
                        inserted_at: current_time(),
                    })?;
                }

                Ok(true)
            })
            .map_err(|e| e.into())
    }
}
