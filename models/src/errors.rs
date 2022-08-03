use diesel::result::Error as DieselError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DBModelError {
    #[error("failed to establish connection pool")]
    ConnectionError(#[from] diesel::ConnectionError),

    #[error("not found record")]
    NotFound,

    #[error("query error")]
    QueryError(DieselError),
}

impl From<DieselError> for DBModelError {
    fn from(err: DieselError) -> Self {
        match err {
            DieselError::NotFound => DBModelError::NotFound,
            _ => DBModelError::QueryError(err),
        }
    }
}
