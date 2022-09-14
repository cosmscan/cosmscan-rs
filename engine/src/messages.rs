use chrono::NaiveDateTime;
use cosmos_client::response;
use cosmos_sdk_proto::cosmos::tx::v1beta1::GetTxResponse;

/// MsgCommittedBlock is a message which indicates committed block.
/// It's intended to be sent to the sender channel of [`Fetcher`].
#[derive(Debug, Clone, PartialEq)]
pub struct MsgCommittedBlock {
    pub block: response::Block,
    pub txs: Vec<response::Transaction>,
    pub events: Vec<response::Event>,
}
