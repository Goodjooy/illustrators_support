use crypto::digest::Digest;
use rocket::futures::stream::select_all;
use sea_orm::{ColumnTrait, Condition, EntityTrait, Set, QueryFilter};

use crate::{database::Database, entity::admins, utils::limit_string::LenLimitedString};

fn crypto_password(paswd: &str) -> String {
    let mut hasher = crypto::sha3::Sha3::keccak256();
    hasher.input_str(paswd);
    hasher.result_str()
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct AdminNew {
    super_identify: String,
    name: LenLimitedString<32>,
    password: LenLimitedString<16>,
}
#[rocket::async_trait]
impl super::TryIntoWithDatabase<admins::ActiveModel> for AdminNew {
    type Error = String;
    async fn try_into(self, _db: &Database) -> Result<admins::ActiveModel, Self::Error> {
        let super_pass = std::env::var("SUPER_ADMIN_PASSWORD").expect("NO Super Admin Exist");
        if super_pass == self.super_identify {
            let res = admins::ActiveModel {
                name: Set(self.name.into()),
                password: Set(crypto_password(self.password.as_ref())),
                ..Default::default()
            };
            Ok(res)
        } else {
            Err("super Admin Password Not Match".to_string())
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct Admin {
    aid: Option<i64>,
    name: LenLimitedString<32>,
    password: LenLimitedString<16>,
}
#[rocket::async_trait]
impl super::SelectBy<admins::Model> for Admin {
    async fn select_by(self, db: &Database) -> Result<Option<admins::Model>, sea_orm::DbErr> {
        let condition = Condition::all()
            .add(admins::Column::Name.eq(self.name.as_ref()))
            .add(admins::Column::Password.eq(crypto_password(self.password.as_ref())));

        admins::Entity::find()
            .filter(condition)
            .one(db.unwarp())
            .await
    }
}
