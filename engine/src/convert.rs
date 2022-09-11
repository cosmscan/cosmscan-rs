use chrono::{NaiveDateTime, Utc};
use cosmos_sdk_proto::cosmos::tx::v1beta1::GetTxResponse;
use cosmoscout_models::models::{block::NewBlock, transaction::NewTransaction};
use tendermint::block::Block;

use crate::utils::current_time;

pub struct NewBlockSchema(NewBlock);
pub struct NewTxSchema(NewTransaction);

impl From<Block> for NewBlockSchema {
    fn from(block: Block) -> Self {
        let date_format_raw_str = "YYYY-MM-DDTHH:mm:ss";
        let date_format = "%Y-%m-%dT%H:%M:%S";
        let block_time_raw_str = block.header.time.to_string();
        let block_time = NaiveDateTime::parse_from_str(
            &block_time_raw_str[..date_format_raw_str.len()],
            date_format,
        )
        .unwrap_or_else(|_| panic!("failed to convert tendermint time to NaiveDateTime"));

        NewBlockSchema(NewBlock {
            chain_id: 0,
            height: block.header.height.into(),
            block_hash: block.header.hash().to_string(),
            prev_hash: block
                .last_commit
                .map(|x| x.block_id.hash.to_string())
                .unwrap_or_default(),
            proposer_address: block.header.proposer_address.to_string(),
            last_commit_hash: block
                .header
                .last_commit_hash
                .unwrap_or_default()
                .to_string(),
            data_hash: block.header.data_hash.unwrap_or_default().to_string(),
            validators_hash: block.header.validators_hash.to_string(),
            next_validators_hash: block.header.next_validators_hash.to_string(),
            consensus_hash: block.header.consensus_hash.to_string(),
            app_hash: block.header.app_hash.to_string(),
            last_result_hash: block
                .header
                .last_results_hash
                .unwrap_or_default()
                .to_string(),
            evidence_hash: block.header.evidence_hash.unwrap_or_default().to_string(),
            block_time,
            inserted_at: current_time(),
        })
    }
}

impl From<&GetTxResponse> for NewTxSchema {
    fn from(tx: &GetTxResponse) -> Self {
        let tx_info = tx.tx.as_ref().unwrap();
        let tx_body = tx_info.body.as_ref().unwrap();
        let tx_response = tx.tx_response.as_ref().unwrap();

        NewTxSchema(NewTransaction {
            chain_id: 0,
            transaction_hash: tx_response.txhash.clone(),
            height: tx_response.height,
            code: tx_response.code as i32,
            code_space: tx_response.codespace.clone(),
            tx_data: tx_response.data.clone(),
            raw_log: tx_response.raw_log.clone(),
            info: tx_response.info.clone(),
            memo: Some(tx_body.memo.clone()),
            gas_wanted: tx_response.gas_wanted,
            gas_used: tx_response.gas_used,
            tx_timestamp: tx_response.timestamp.clone(),
            inserted_at: NaiveDateTime::from_timestamp(Utc::now().timestamp(), 0),
        })
    }
}

impl From<NewBlockSchema> for NewBlock {
    fn from(block: NewBlockSchema) -> Self {
        block.0
    }
}

impl From<NewTxSchema> for NewTransaction {
    fn from(tx: NewTxSchema) -> Self {
        tx.0
    }
}
