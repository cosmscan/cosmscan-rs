use diesel::result::Error as DieselError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("failed to establish connection pool")]
    ConnectionError(#[from] diesel::ConnectionError),

    #[error("you forgot to connect to the database")]
    ClientDoesntExists,

    #[error("not found record")]
    NotFound,

    #[error("query error")]
    QueryError(DieselError),
}

impl From<DieselError> for Error {
    fn from(err: DieselError) -> Self {
        match err {
            DieselError::NotFound => Error::NotFound,
            _ => Error::QueryError(err),
        }
    }
}
