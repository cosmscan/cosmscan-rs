use std::str::Utf8Error;

use tendermint_rpc::error::ErrorDetail;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("failed to convert bytes to the utf8 string")]
    Utf8Error(#[from] Utf8Error),

    #[error("Recevied failed messsage from tendermint rpc server")]
    RPCError(tendermint_rpc::Error),

    #[error("Received failed message from cosmos gRPC server")]
    GRPCError(#[from] tonic::transport::Error),

    #[error("tonic status failed")]
    TonicStatusError(#[from] tonic::Status),

    #[error("serde json error")]
    InvalidJSONError(#[from] serde_json::Error),

    #[error("failed to call the rest api with reqwest")]
    RestAPIERror(#[from] reqwest::Error),

    #[error("unknown server error")]
    UnknownServerError(tendermint_rpc::Error),
}

impl From<tendermint_rpc::Error> for Error {
    fn from(err: tendermint_rpc::Error) -> Self {
        match err.clone() {
            tendermint_rpc::Error(ErrorDetail::Response(_), _) => Error::RPCError(err),
            _ => Error::UnknownServerError(err),
        }
    }
}
