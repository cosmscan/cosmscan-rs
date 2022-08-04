use std::str::FromStr;

use chrono::{NaiveDateTime, Utc};
use cosmoscout_models::models::block::NewBlock;
use log::info;
use tendermint::block::Block;

pub struct NewBlockSchema(NewBlock);

impl From<Block> for NewBlockSchema {
    fn from(block: Block) -> Self {
        let date_format_raw_str = "YYYY-MM-DDTHH:mm:ss";
        let date_format = "%Y-%m-%dT%H:%M:%S";
        let block_time_raw_str = block.header.time.to_string();
        let block_time = 
            NaiveDateTime::parse_from_str(&block_time_raw_str[..date_format_raw_str.len()], date_format)
            .unwrap_or_else(|_| panic!("failed to convert tendermint time to NaiveDateTime"));

        NewBlockSchema(NewBlock {
            chain_id: 0,
            height: block.header.height.into(),
            block_hash: block.header.hash().to_string(),
            prev_hash: block.last_commit.map(|x| x.block_id.hash.to_string()).unwrap_or_default(),
            proposer_address: block.header.proposer_address.to_string(),
            last_commit_hash: block.header.last_commit_hash.unwrap_or_default().to_string(),
            data_hash: block.header.data_hash.unwrap_or_default().to_string(),
            validators_hash: block.header.validators_hash.to_string(),
            next_validators_hash: block.header.next_validators_hash.to_string(),
            consensus_hash: block.header.consensus_hash.to_string(),
            app_hash: block.header.app_hash.to_string(),
            last_result_hash: block.header.last_results_hash.unwrap_or_default().to_string(),
            evidence_hash: block.header.evidence_hash.unwrap_or_default().to_string(),
            block_time: block_time,
            inserted_at: NaiveDateTime::from_timestamp(Utc::now().timestamp(), 0),
        })
    }
}

impl From<NewBlockSchema> for NewBlock {
    fn from(block: NewBlockSchema) -> Self {
        block.0
    }
}

#[cfg(test)]
mod tests {
    use chrono::{NaiveDateTime, NaiveDate};

    #[test]
    fn parse_tendermint_time() {
        let date_format = "YYYY-MM-DDTHH:mm:ss";
        let time = &"2022-08-04T02:08:43.1201678Z"[..date_format.len()];
        let parsed = NaiveDateTime::parse_from_str(time, "%Y-%m-%dT%H:%M:%S");
        assert_eq!(parsed, Ok(NaiveDate::from_ymd(2022, 8, 4).and_hms(2, 8, 43)));
    }
}