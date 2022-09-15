use std::{fs, sync::Arc};

use cosmscan_models::{config::DBConfig, db::BackendDB, storage::PersistenceStorage};
use serde::Deserialize;

mod errors;
mod handlers;
mod router;
pub mod server;

type GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;

pub struct AppState {
    pub storage: Arc<PersistenceStorage<BackendDB>>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Config {
    pub db: DBConfig,
    pub server: ServerConfig,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

impl AppState {
    fn new(storage: Arc<PersistenceStorage<BackendDB>>) -> Self {
        Self { storage }
    }
}

impl Config {
    pub fn from_file(file: String) -> Result<Self, GenericError> {
        let raw_config = fs::read_to_string(file)?;
        let config: Config = toml::from_str(raw_config.as_str()).unwrap();
        Ok(config)
    }
}
