use crate::{
    entity::users,
    utils::data_structs::{crypto_string::CryptoString, RangeLimitString},
};

use sea_orm::{ColumnTrait, Condition, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};

use crate::entity;

use super::{CheckExits, SelectBy, TryIntoWithDatabase};

#[derive(Serialize, Deserialize, Clone)]
pub struct UserLogin {
    pub id: Option<i64>,
    pub name: Option<RangeLimitString<4, 32>>,
    pub qq: i64,
    #[serde(alias = "pwd")]
    pub password: CryptoString<6, 16>,
}
#[rocket::async_trait]
impl SelectBy<entity::users::Model> for UserLogin {
    async fn select_by(
        self,
        db: &crate::database::Database,
    ) -> Result<Option<entity::users::Model>, sea_orm::DbErr> {
        entity::users::Entity::find()
            .filter(
                Condition::all()
                    .add(entity::users::Column::Qq.eq(self.qq))
                    .add(
                        entity::users::Column::Password
                            .eq::<String>(self.password.into_crypto().into()),
                    ),
            )
            .one(db.unwarp())
            .await
    }
}
#[async_trait]
impl CheckExits for UserLogin {
    async fn exist(&self, db: &crate::database::Database) -> Result<bool, sea_orm::DbErr> {
        users::Entity::find()
            .filter(
                Condition::all()
                    .add(users::Column::Qq.eq(self.qq))
                    .add(users::Column::Password.eq(self.password.as_ref())),
            )
            .one(db.unwarp())
            .await
            .and_then(|o| Ok(o.is_some()))
    }
}

impl From<entity::users::Model> for UserLogin {
    fn from(src: entity::users::Model) -> Self {
        Self {
            id: Some(src.id),
            name: RangeLimitString::try_from(src.name).ok(),
            qq: src.qq,
            password: CryptoString::try_from(src.password).unwrap(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UserNew {
    pub name: RangeLimitString<4, 32>,
    pub qq: i64,
    #[serde(alias = "pwd")]
    pub password: CryptoString<6, 16>,
    #[serde(alias = "code")]
    invite_code: RangeLimitString<8, 36>,
}

#[async_trait]
impl TryIntoWithDatabase<users::ActiveModel> for UserNew {
    type Error = String;

    async fn try_into_with_db(
        self,
        db: &crate::database::Database,
    ) -> Result<users::ActiveModel, Self::Error> {
        let code = self.invite_code.clone().into();
        let res = code.select_by(db).await.or_else(|e| Err(e.to_string()))?;
        if let Some(_a) = res {
            let res = entity::users::ActiveModel {
                name: Set(self.name.into()),
                qq: Set(self.qq),
                password: Set(self.password.into_crypto().into()),
                ..Default::default()
            };
            Ok(res)
        } else {
            Err("Invite Code Not Match".to_string())
        }
    }
}
