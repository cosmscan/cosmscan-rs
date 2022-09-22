use std::{fs, sync::Arc};

use cosmscan_models::{config::DBConfig, db::BackendDB, storage::PersistenceStorage};
use hyper::Response;
use resputil::ResponseBuilder;
use route_recognizer::Params;
use serde::Deserialize;

mod errors;
mod handlers;
mod responses;
mod resputil;
mod router;
pub mod server;

type GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;

pub struct AppState {
    pub storage: Arc<PersistenceStorage<BackendDB>>,
    pub params: Params,
    pub resp_builder: Arc<ResponseBuilder>,
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
    pub allowed_host: String,
}

impl AppState {
    fn new(
        storage: Arc<PersistenceStorage<BackendDB>>,
        params: Params,
        resp_builder: Arc<ResponseBuilder>,
    ) -> Self {
        Self {
            storage,
            params,
            resp_builder,
        }
    }
}

impl Config {
    pub fn from_file(file: String) -> Result<Self, GenericError> {
        let raw_config = fs::read_to_string(file)?;
        let config: Config = toml::from_str(raw_config.as_str()).unwrap();
        Ok(config)
    }
}
