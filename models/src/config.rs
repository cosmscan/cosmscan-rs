use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct DBConfig {
    pub host: String,
    pub port: u32,
    pub user: String,
    pub password: String,
    pub database: String,
}

impl DBConfig {
    #[cfg(test)]
    pub(crate) fn default() -> Self {
        DBConfig { 
            host: "localhost".to_string(), 
            port: 5432, 
            user: "cosmoscout".to_string(), 
            password: "cosmoscout".to_string(), 
            database: "cosmoscout".to_string(),
        }
    }
}