use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct DBConfig {
    pub host: String,
    pub port: u32,
    pub user: String,
    pub password: String,
    pub database: String,
}
