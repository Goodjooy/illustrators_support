use std::{env, error::Error};

use sea_orm::{ConnectOptions, Database, DatabaseConnection};

pub async fn connect_db() -> Result<DatabaseConnection, Box<dyn Error>> {
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
        .sqlx_logging(true)
        .min_connections(2);

    let db = Database::connect(ops).await?;

    Ok(db)
}
