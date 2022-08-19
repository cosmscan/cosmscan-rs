use chrono::{NaiveDateTime, Utc};
use cosmoscout_models::models::{block::NewBlock, transaction::NewTransaction};
use tendermint::{abci::Code, block::Block};
use tendermint_rpc::endpoint::tx;

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

impl From<tx::Response> for NewTxSchema {
    fn from(tx: tx::Response) -> Self {
        let code = match tx.tx_result.code {
            Code::Ok => 0,
            Code::Err(err) => err,
        };

        NewTxSchema(NewTransaction {
            chain_id: 0,
            transaction_hash: tx.hash.to_string(),
            height: tx.height.into(),
            code: code as i32,
            code_space: tx.tx_result.codespace.to_string(),
            tx_data: tx.tx_result.data.value().to_owned(),
            raw_log: Some(tx.tx_result.log.value().to_owned()),
            info: Some(tx.tx_result.info.to_string()),
            memo: None,
            gas_wanted: tx.tx_result.gas_wanted.value() as i64,
            gas_used: tx.tx_result.gas_used.value() as i64,
            tx_date: None,
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

#[cfg(test)]
mod tests {
    use chrono::{NaiveDate, NaiveDateTime};

    #[test]
    fn parse_tendermint_time() {
        let date_format = "YYYY-MM-DDTHH:mm:ss";
        let time = &"2022-08-04T02:08:43.1201678Z"[..date_format.len()];
        let parsed = NaiveDateTime::parse_from_str(time, "%Y-%m-%dT%H:%M:%S");
        assert_eq!(
            parsed,
            Ok(NaiveDate::from_ymd(2022, 8, 4).and_hms(2, 8, 43))
        );
    }
}
