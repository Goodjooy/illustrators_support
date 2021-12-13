use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};

use crate::{
    entity::invites,
    utils::data_structs::{RangeLimitString, RangeLimitVec},
};

use super::{SelectBy, TryIntoWithDatabase};

#[derive(Serialize, Deserialize, Clone)]
pub struct InviteCodeNew {
    codes: RangeLimitVec<RangeLimitString<8, 36>, 1, 3>,
}

#[async_trait]
impl TryIntoWithDatabase<invites::ActiveModel> for RangeLimitString<8, 36> {
    type Error = String;

    async fn try_into_with_db(
        self,
        db: &crate::database::Database,
    ) -> Result<invites::ActiveModel, Self::Error> {
        let temp = invites::Entity::find()
            .filter(invites::Column::Code.eq(self.as_ref().as_str()))
            .one(db.unwarp())
            .await
            .or_else(|e| Err(e.to_string()))?;
        if let None = temp {
            Ok(invites::ActiveModel {
                code: Set(self.into()),
                ..Default::default()
            })
        } else {
            Err("Code Exist".to_string())
        }
    }
}
#[async_trait]
impl TryIntoWithDatabase<Vec<invites::ActiveModel>> for InviteCodeNew {
    type Error = String;

    async fn try_into_with_db(
        self,
        db: &crate::database::Database,
    ) -> Result<Vec<invites::ActiveModel>, Self::Error> {
        let mut res = Vec::with_capacity(self.codes.len());

        for code in self.codes.into() {
            let t = TryIntoWithDatabase::<invites::ActiveModel>::try_into_with_db(code, &db).await;
            if let Ok(r) = t {
                res.push(r);
            }
        }

        Ok(res)
    }
}

impl From<invites::Model> for String {
    fn from(m: invites::Model) -> Self {
        m.code
    }
}
#[async_trait]
impl SelectBy<invites::Model> for String {
    async fn select_by(
        self,
        db: &crate::database::Database,
    ) -> Result<Option<invites::Model>, sea_orm::DbErr> {
        invites::Entity::find()
            .filter(invites::Column::Code.eq(self))
            .one(db.unwarp())
            .await
    }
}

#[derive(Deserialize, Serialize)]
pub struct CodeSimple {
    pub id: i64,
    pub value: String,
}

impl From<invites::Model> for CodeSimple {
    fn from(d: invites::Model) -> Self {
        Self {
            id: d.id,
            value: d.code,
        }
    }
}
