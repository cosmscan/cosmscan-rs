use cosmoscout_models::errors::DBModelError;
use tendermint_rpc::{ResponseError, error::ErrorDetail};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FetchError {
    #[error("one of action for fetching transaction failed")]
    FetchingTransactionFailed,

    #[error("database model error")]
    DBError(#[from] DBModelError),

    #[error("failed to conenct with tendermint rpc client")]
    RPCError(tendermint_rpc::Error),
    
    #[error("unknown server error")]
    UnknownServerError(tendermint_rpc::Error)
}

impl From<tendermint_rpc::Error> for FetchError {
    fn from(err: tendermint_rpc::Error) -> Self {
        match err.clone() {
            tendermint_rpc::Error(ErrorDetail::Response(_), _) => {
                FetchError::RPCError(err)
            },
            _ => FetchError::UnknownServerError(err)
        }
    }
}
