use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("method is not allowed {0}")]
    MethodNotAllowed(String),
}
