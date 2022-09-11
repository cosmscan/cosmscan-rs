use chrono::NaiveDateTime;
use cosmos_sdk_proto::cosmos::tx::v1beta1::GetTxResponse;

/// Represents a event occurred in the cosmos blockchain.
/// It's usually emitted by executing transaction & proposing block.
#[derive(Debug, Clone, PartialEq)]
pub struct RawEvent {
    pub tx_type: i16,
    pub tx_hash: Option<String>,
    pub event_type: String,
    pub event_key: String,
    pub event_value: String,
    pub indexed: bool,
}

/// Represents a transaction as readable and inlined format.
#[derive(Debug, Clone, PartialEq)]
pub struct RawTx {
    pub transaction_hash: String,
    pub height: i64,
    pub code: i32,
    pub code_space: String,
    pub tx_data: String,
    pub raw_log: String,
    pub info: String,
    pub memo: Option<String>,
    pub gas_wanted: i64,
    pub gas_used: i64,
    pub tx_timestamp: String,
}

/// Prepresents a block as inlined format.
#[derive(Debug, Clone, PartialEq)]
pub struct RawBlock {
    pub height: i64,
    pub block_hash: String,
    pub prev_hash: String,
    pub proposer_address: String,
    pub last_commit_hash: String,
    pub data_hash: String,
    pub validators_hash: String,
    pub next_validators_hash: String,
    pub consensus_hash: String,
    pub app_hash: String,
    pub last_result_hash: String,
    pub evidence_hash: String,
    pub block_time: NaiveDateTime,
}

impl From<tendermint::block::Block> for RawBlock {
    fn from(block: tendermint::block::Block) -> Self {
        let date_format_raw_str = "YYYY-MM-DDTHH:mm:ss";
        let date_format = "%Y-%m-%dT%H:%M:%S";
        let block_time_raw_str = block.header.time.to_string();
        let block_time = NaiveDateTime::parse_from_str(
            &block_time_raw_str[..date_format_raw_str.len()],
            date_format,
        )
        .unwrap_or_else(|_| panic!("failed to convert tendermint time to NaiveDateTime"));

        RawBlock {
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
        }
    }
}

impl From<&GetTxResponse> for RawTx {
    fn from(tx: &GetTxResponse) -> Self {
        let tx_info = tx.tx.as_ref().unwrap();
        let tx_body = tx_info.body.as_ref().unwrap();
        let tx_response = tx.tx_response.as_ref().unwrap();

        RawTx {
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
        }
    }
}
