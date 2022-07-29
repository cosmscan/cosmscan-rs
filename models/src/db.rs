use tokio_postgres::{NoTls, Client};

use crate::config::DBConfig;

pub struct Database {
    pub(crate) config: DBConfig,
    pub(crate) client: Option<Client>,
}

impl Database {
    pub fn new(config: DBConfig) -> Self {
        return Database {
            config: config,
            client: None,
        }
    }

    pub async fn connect(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let config_names = format!(
            "host={} port={} user={} password={} dbname={}",
            self.config.host,
            self.config.port,
            self.config.user,
            self.config.password,
            self.config.database,
        );

        let (client, connection) = tokio_postgres::connect(config_names.as_str(),NoTls).await?;
        connection.await?;

        self.client = Some(client);

        Ok(())
    }

    pub async fn close(&self) {
    
    }
}

