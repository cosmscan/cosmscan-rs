use diesel::{r2d2::ConnectionManager, PgConnection};
use r2d2::{Pool, PooledConnection};

use crate::config::DBConfig;

pub trait Database {
    fn connect(&mut self) -> Result<bool, Box<dyn std::error::Error>>;
    fn conn(&self) -> Option<PooledConnection<ConnectionManager<PgConnection>>>;
}

pub struct BackendDB {
    pub config: DBConfig,
    pub client: Option<Pool<ConnectionManager<PgConnection>>>,
}

impl BackendDB {
    pub fn new(config: DBConfig) -> Self {
        return BackendDB {
            config: config,
            client: None,
        }
    }
}

impl Database for BackendDB {
    fn connect(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let db_url = format!("postgres://{}:{}@{}:{}/{}",
            self.config.user,
            self.config.password,
            self.config.host,
            self.config.port,
            self.config.database,
        );

        let manager = ConnectionManager::<PgConnection>::new(db_url.clone());
        let pool = Pool::new(manager)
            .expect(format!("failed to conect to the database: {}", db_url).as_str());
        
        self.client = Some(pool);

        Ok(true)
    }

    fn conn(&self) -> Option<PooledConnection<ConnectionManager<PgConnection>>> {
        self.client.as_ref().map(|p| p.get().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn connect_database() {
        let mut db = BackendDB::new(DBConfig::default());
        let result = db.connect();
        assert_eq!(result.unwrap(), true);
    }
}