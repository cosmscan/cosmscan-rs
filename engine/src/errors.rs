use std::str::Utf8Error;

use tendermint_rpc::error::ErrorDetail;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("one of action for fetching transaction failed")]
    FetchingTransactionFailed,

    #[error("database model error")]
    DBError(#[from] cosmoscout_models::errors::Error),

    #[error("failed to conenct with tendermint rpc client")]
    RPCError(tendermint_rpc::Error),

    #[error("failed to connect to the cosmos grpc server")]
    GRPCError(#[from] tonic::transport::Error),

    #[error("tonic status failed")]
    TonicStatusError(#[from] tonic::Status),

    #[error("serde json error")]
    InvalidJSONError(#[from] serde_json::Error),

    #[error("failed to call the rest api with reqwest")]
    RestAPIERror(#[from] reqwest::Error),

    #[error("unknown server error")]
    UnknownServerError(tendermint_rpc::Error),

    #[error("failed to convert attribute to the string")]
    Utf8ConversionError(#[from] Utf8Error),

    #[error("start block must be greater than 0")]
    StartBlockMustBeGreaterThanZero,

    #[error("Unexpected error")]
    UnexpectedError,

    #[error("unknown error ${0}")]
    Other(String)
}

impl From<tendermint_rpc::Error> for Error {
    fn from(err: tendermint_rpc::Error) -> Self {
        match err.clone() {
            tendermint_rpc::Error(ErrorDetail::Response(_), _) => Error::RPCError(err),
            _ => Error::UnknownServerError(err),
        }
    }
}
