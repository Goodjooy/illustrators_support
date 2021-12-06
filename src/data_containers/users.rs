use crate::{data_containers::TryIntoWithDatabase, database::Database};
use std::env;

use rocket::http::Cookie;
use sea_orm::{ColumnTrait, Condition, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};

use crate::{entity, utils::limit_string::LenLimitedString};

use super::SelectBy;

#[derive(Serialize, Deserialize, Clone)]
pub struct UserLogin {
    pub id: Option<i64>,
    pub name: LenLimitedString<32>,
    pub qq: i64,
}
#[rocket::async_trait]
impl SelectBy<entity::users::Model> for UserLogin {
    async fn select_by(
        self,
        db: &crate::database::Database,
    ) -> Result<Option<entity::users::Model>, sea_orm::DbErr> {
        if let Some(id) = self.id {
            entity::users::Entity::find_by_id(id).one(db.unwarp()).await
        } else {
            entity::users::Entity::find()
                .filter(
                    Condition::all()
                        .add(entity::users::Column::Name.eq(&*self.name))
                        .add(entity::users::Column::Qq.eq(self.qq)),
                )
                .one(db.unwarp())
                .await
        }
    }
}

impl From<entity::users::Model> for UserLogin {
    fn from(src: entity::users::Model) -> Self {
        Self {
            id: Some(src.id),
            name: src.name.try_into().unwrap(),
            qq: src.qq,
        }
    }
}

impl UserLogin {
    pub fn to_cookie(self, cookie_name: &str) -> Cookie {
        Cookie::new(cookie_name, serde_json::to_string(&self).unwrap())
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UserNew {
    pub name: LenLimitedString<32>,
    pub qq: i64,
    invite_code: LenLimitedString<36>,
}
#[rocket::async_trait]
impl TryIntoWithDatabase<entity::users::ActiveModel> for UserNew {
    type Error = String;

    async fn try_into(self, _db: &Database) -> Result<entity::users::ActiveModel, Self::Error> {
        dotenv::dotenv().ok();
        let invite_code = env::var("INVITE_CODE").expect("No invite_code exist");
        if &*self.invite_code == &invite_code {
            let res = entity::users::ActiveModel {
                name: Set(self.name.into()),
                qq: Set(self.qq),
                ..Default::default()
            };
            Ok(res)
        } else {
            Err("Invite Code Not Match".to_string())
        }
    }
}