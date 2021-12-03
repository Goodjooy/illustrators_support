use std::{env, error::Error};

use sea_orm::{ConnectOptions, DatabaseConnection};

pub struct Database {
    db: DatabaseConnection,
}

impl AsRef<DatabaseConnection> for Database {
    fn as_ref(&self) -> &DatabaseConnection {
        &self.db
    }
}

impl<'a> Into<& 'a DatabaseConnection> for & 'a Database {
    fn into(self) -> & 'a DatabaseConnection {
        &self.db
    }
}

impl Database {
    pub async fn connect_db() -> Result<Self, Box<dyn Error>> {
        dotenv::dotenv().ok();
        let db_url =
            env::var("DATABASE_URL").or_else(|e| Err(format!("DATABASE_URL Not Found | {}", e)))?;
        let max_conn = env::var("MAX_CONN")
            .unwrap_or("32".into())
            .trim()
            .parse()
            .unwrap_or(32u32);

        let mut ops = ConnectOptions::new(db_url);
        ops.max_connections(max_conn)
            //.sqlx_logging(true)
            .min_connections(2);

        let db = sea_orm::Database::connect(ops).await?;

        Ok(Self { db })
    }
    pub fn unwarp(&self)->&DatabaseConnection{
        &self.db
    }
}
