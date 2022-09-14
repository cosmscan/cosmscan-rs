use cosmscan_models::config::DBConfig;
use serde::Deserialize;
use std::error::Error;
use std::fs;

#[derive(Debug, PartialEq, Deserialize)]
pub struct Config {
    pub engine: EngineConfig,
    pub fetcher: FetcherConfig,
    pub chain: ChainConfig,
    pub db: DBConfig,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct ChainConfig {
    pub chain_id: String,
    pub chain_name: String,
    pub icon_url: Option<String>,
    pub website: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct FetcherConfig {
    pub tendermint_rpc_endpoint: String,
    pub grpc_endpoint: String,
    pub rest_api_endpoint: String,
    pub start_block: i64,
    pub try_resume_from_db: bool,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct EngineConfig {
    pub fetcher_account_enabled: bool,
}

impl Config {
    pub fn from_file(file: String) -> Result<Self, Box<dyn Error>> {
        let raw_config = fs::read_to_string(file)?;
        let config: Config = toml::from_str(raw_config.as_str()).unwrap();
        Ok(config)
    }
}
