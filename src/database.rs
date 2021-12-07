use crate::{data_containers::TryIntoWithDatabase, entity::invites};
use sea_orm::{ConnectOptions, DatabaseConnection, DbErr, EntityTrait};
use std::error::Error;

use crate::utils::{
    config::{DbConfig, DefaultInviteCodeConfig},
    measureable::Measurable,
};

pub struct Database {
    db: DatabaseConnection,
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
        dotenv::dotenv().ok();
        let db_url = db_config.url.clone();
        let max_conn = db_config.max_conn;
        let min_conn = db_config.min_conn;

        let mut ops = ConnectOptions::new(db_url);
        ops.max_connections(max_conn).min_connections(min_conn);

        let db = sea_orm::Database::connect(ops).await?;

        Ok(Self { db })
    }
    pub fn unwarp(&self) -> &DatabaseConnection {
        &self.db
    }

    pub async fn add_default_code(
        &self,
        code_config: DefaultInviteCodeConfig,
    ) -> Result<(), DbErr> {
        let mut res = Vec::with_capacity(code_config.codes.size());

        for code in code_config.codes {
            let re = code.try_into_with_db(&self).await;
            if let Ok(r) = re {
                res.push(r);
            }
        }
        invites::Entity::insert_many(res).exec(&self.db).await?;

        Ok(())
    }
}
