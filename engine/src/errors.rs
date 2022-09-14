use std::str::Utf8Error;

use tendermint_rpc::error::ErrorDetail;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("one of action for fetching transaction failed")]
    FetchingTransactionFailed,

    #[error("cosmos client error occurred")]
    CosmosClientError(#[from] cosmos_client::errors::Error),

    #[error("database model error")]
    DBError(#[from] cosmscan_models::errors::Error),

    #[error("serde json error")]
    InvalidJSONError(#[from] serde_json::Error),

    #[error("unknown server error")]
    UnknownServerError(tendermint_rpc::Error),

    #[error("failed to convert attribute to the string")]
    Utf8ConversionError(#[from] Utf8Error),

    #[error("start block must be greater than 0")]
    StartBlockMustBeGreaterThanZero,

    #[error("Unexpected error")]
    UnexpectedError,

    #[error("unknown error ${0}")]
    Other(String),
}
