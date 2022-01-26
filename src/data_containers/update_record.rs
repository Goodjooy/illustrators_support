use chrono::NaiveDateTime;
use sea_orm::{EntityTrait, QueryFilter};

use crate::{
    database::update_bound::record_condition, utils::data_structs::header_info::HeaderInfo,
};

use super::SelectBy;

pub struct RecordUnit(pub NaiveDateTime);

#[derive(sea_orm::FromQueryResult)]
struct RecordMid {
    #[allow(dead_code)]
    id: i64,
}

#[rocket::async_trait]
impl SelectBy<usize> for RecordUnit {
    async fn select_by(
        self,
        db: &crate::database::Database,
    ) -> Result<Option<usize>, sea_orm::DbErr> {
        use crate::entity::update_record;
        update_record::Entity::find()
            .filter(record_condition(None, self.0))
            .into_model::<RecordMid>()
            .all(db.unwarp())
            .await
            .and_then(|v| Ok(if v.len() == 0 { None } else { Some(v.len()) }))
    }
}

crate::header_captures!(pub LastUpdate : "Last-Update");

pub enum LoadUpdateRecordErr {
    Parse(chrono::format::ParseError),
    NotExist,
}

impl From<chrono::format::ParseError> for LoadUpdateRecordErr {
    fn from(s: chrono::format::ParseError) -> Self {
        Self::Parse(s)
    }
}

impl<'s> TryInto<NaiveDateTime> for HeaderInfo<'s, LastUpdate> {
    type Error = LoadUpdateRecordErr;

    fn try_into(self) -> Result<NaiveDateTime, Self::Error> {
        self.get_one()
            .ok_or(LoadUpdateRecordErr::NotExist)
            .and_then(|h| {
                chrono::NaiveDateTime::parse_from_str(h, "%Y-%m-%d %H:%M:%S")
                    .or_else(|e| Err(e.into()))
            })
    }
}
