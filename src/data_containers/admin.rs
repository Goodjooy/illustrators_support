use sea_orm::{ColumnTrait, Condition, EntityTrait, QueryFilter, Set};

use crate::{
    database::Database,
    entity::admins,
    utils::data_structs::{crypto_string::CryptoString, RangeLimitString},
};

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct AdminNew {
    #[serde(alias = "sid")]
    #[serde(alias = "spwd")]
    pub super_identify: String,
    pub name: RangeLimitString<1, 32>,
    #[serde(alias = "pwd")]
    pub password: CryptoString<6, 16>,
}

impl super::TryIntoWithConfig<admins::ActiveModel> for AdminNew {
    type Error = String;
    fn try_into_with_config(
        self,
        config: &crate::utils::config::Config,
    ) -> Result<admins::ActiveModel, Self::Error> {
        let super_pass = &config.auth.super_admin_auth;
        if super_pass == &self.super_identify {
            let res = admins::ActiveModel {
                name: Set(self.name.into()),
                password: Set(self.password.into_crypto().into()),
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
    pub aid: Option<i64>,
    pub name: RangeLimitString<1, 32>,
    #[serde(alias = "pwd")]
    pub password: CryptoString<6, 16>,
}
#[rocket::async_trait]
impl super::SelectBy<admins::Model> for Admin {
    async fn select_by(self, db: &Database) -> Result<Option<admins::Model>, sea_orm::DbErr> {
        let condition = Condition::all()
            .add(admins::Column::Name.eq(self.name.as_ref().as_str()))
            .add(admins::Column::Password.eq::<String>(self.password.into_crypto().into()));

        admins::Entity::find()
            .filter(condition)
            .one(db.unwarp())
            .await
    }
}

impl From<admins::Model> for Admin {
    fn from(m: admins::Model) -> Self {
        Self {
            aid: Some(m.id),
            name: RangeLimitString::try_from(m.name).unwrap(),
            password: CryptoString::try_from(m.password).unwrap(),
        }
    }
}
