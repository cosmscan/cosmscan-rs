use cosmoscout_models::config::DBConfig;
use serde::Deserialize;
use std::error::Error;
use std::fs;

#[derive(Debug, PartialEq, Deserialize)]
pub struct Config {
    pub fetcher: FetcherConfig,
    pub db: DBConfig,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct FetcherConfig {
    pub tendermint_rpc: String,
    pub cosmos_rest: String,
    pub start_block: u32,
    pub try_resume_from_db: bool,
}

impl Config {
    pub fn from_file(file: String) -> Result<Self, Box<dyn Error>> {
        let raw_config = fs::read_to_string(file)?;
        let config: Config = toml::from_str(raw_config.as_str()).unwrap();
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read() {
        let config = Config::from_file("../config.toml".to_string()).unwrap();
        assert_eq!(
            config.fetcher,
            FetcherConfig {
                tendermint_rpc: "http://localhost:26657/".to_string(),
                cosmos_rest: "http://localhost:1317/".to_string(),
                start_block: 1,
                try_resume_from_db: true,
            }
        );
    }
}
