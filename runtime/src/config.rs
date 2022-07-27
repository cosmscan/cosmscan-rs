use serde::Deserialize;
use std::error::Error;
use std::fs;

#[derive(Debug, PartialEq, Deserialize)]
pub struct RuntimeConfig {
    pub tendermint_rpc: String,
    pub cosmos_rest: String,
}

impl RuntimeConfig {
    pub fn from_file(file: String) -> Result<Self, Box<dyn Error>> {
        let raw_config = fs::read_to_string(file)?;
        let config:RuntimeConfig = toml::from_str(raw_config.as_str()).unwrap();
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read() {
        let config = RuntimeConfig::from_file("../config.toml".to_string()).unwrap();
        assert_eq!(config, RuntimeConfig{
            tendermint_rpc: "http://localhost:26657/".to_string(),
            cosmos_rest: "http://localhost:1317/".to_string()
        })
    }
}