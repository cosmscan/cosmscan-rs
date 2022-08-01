use thiserror::Error;

#[derive(Error, Debug)]
pub enum DBModelError {
    #[error("failed to establish connection pool")]
    ConnectionError(#[from] diesel::ConnectionError),

    #[error("query error")]
    QueryError(#[from] diesel::result::Error)
}