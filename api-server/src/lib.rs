pub mod server;
mod router;

type GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;