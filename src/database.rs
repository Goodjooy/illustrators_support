/**
 * @Author: Your name
 * @Date:   2021-12-01 19:07:55
 * @Last Modified by:   Your name
 * @Last Modified time: 2021-12-11 13:18:47
 */
use crate::{
    data_containers::TryIntoWithDatabase,
    entity::invites,
    utils::{
        config::{database::DbConfig, invite_code::DefaultInviteCodeConfig},
        data_structs::measureable::Measurable,
    },
};
use log::info;
use sea_orm::{ConnectOptions, DatabaseConnection, DbErr, EntityTrait};
use std::error::Error;

pub struct Database {
    db: DatabaseConnection,
}

impl From<DatabaseConnection> for Database {
    fn from(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

impl AsRef<DatabaseConnection> for Database {
    fn as_ref(&self) -> &DatabaseConnection {
        &self.db
    }
}

impl<'a> Into<&'a DatabaseConnection> for &'a Database {
    fn into(self) -> &'a DatabaseConnection {
        &self.db
    }
}

impl Database {
    pub async fn connect_db(db_config: &DbConfig) -> Result<Self, Box<dyn Error>> {
        log::info!("Start Conncet  to database");

        dotenv::dotenv().ok();
        let db_url = db_config.url.clone();
        let max_conn = db_config.max_conn;
        let min_conn = db_config.min_conn;

        let mut ops = ConnectOptions::new(db_url);
        ops.max_connections(max_conn)
            .min_connections(min_conn)
            .sqlx_logging(db_config.db_log);

        let db = sea_orm::Database::connect(ops).await?;

        info!("Connect to database success");
        Ok(Self { db })
    }
    pub fn unwarp(&self) -> &DatabaseConnection {
        &self.db
    }

    pub async fn add_default_code(
        &self,
        code_config: DefaultInviteCodeConfig,
    ) -> Result<(), DbErr> {
        info!("Adding init invite code to database");
        let mut res = Vec::with_capacity(code_config.codes.size());

        for code in code_config.codes {
            info!("adding invite code to databse {}", &code);
            let re = code.try_into_with_db(&self).await;
            if let Ok(r) = re {
                res.push(r);
            }
        }
        if res.len() != 0 {
            invites::Entity::insert_many(res).exec(&self.db).await?;
        }
        info!("Adding init invite code to database DONE");
        Ok(())
    }
}
