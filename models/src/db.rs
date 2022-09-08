use diesel::{r2d2::ConnectionManager, PgConnection};
use r2d2::{Pool, PooledConnection};

use crate::config::DBConfig;

pub trait Database {
    fn connect(&mut self) -> bool;
    fn conn(&self) -> Option<PooledConnection<ConnectionManager<PgConnection>>>;
}

pub struct BackendDB {
    pub config: DBConfig,
    pub client: Option<Pool<ConnectionManager<PgConnection>>>,
}

impl BackendDB {
    pub fn new(config: DBConfig) -> Self {
        BackendDB {
            config,
            client: None,
        }
    }
}

impl Database for BackendDB {
    fn connect(&mut self) -> bool {
        let db_url = format!(
            "postgres://{}:{}@{}:{}/{}",
            self.config.user,
            self.config.password,
            self.config.host,
            self.config.port,
            self.config.database,
        );

        let manager = ConnectionManager::<PgConnection>::new(db_url.clone());
        let pool = Pool::new(manager)
            .unwrap_or_else(|_| panic!("failed to conect to the database: {}", db_url));

        self.client = Some(pool);

        true
    }

    fn conn(&self) -> Option<PooledConnection<ConnectionManager<PgConnection>>> {
        self.client.as_ref().map(|p| p.get().unwrap())
    }
}