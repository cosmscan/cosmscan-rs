use cosmos_client::response;

/// MsgCommittedBlock is a message which indicates committed block.
/// It's intended to be sent to the sender channel of [`Fetcher`].
#[derive(Debug, Clone, PartialEq)]
pub struct MsgCommittedBlock {
    pub block: response::Block,
    pub txs: Vec<response::Transaction>,
    pub events: Vec<response::Event>,
}
