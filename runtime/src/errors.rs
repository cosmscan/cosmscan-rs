use cosmoscout_models::errors::DBModelError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FetchError {
    #[error("one of action for fetching transaction failed")]
    FetchingTransactionFailed,

    #[error("database model error")]
    DBError(#[from] DBModelError),

    #[error("failed to conenct with tendermint rpc client")]
    RPCError(#[from] tendermint_rpc::Error)
}