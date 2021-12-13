use sea_orm::{ColumnTrait, Condition, Set};
use serde::Deserialize;

use crate::{
    entity::{file_stores, illustrator_acts},
    utils::data_structs::MaxLimitString,
};

#[derive(FromForm, Deserialize)]
pub struct ArtNew {
    #[field(name = uncased("img"))]
    #[field(name = uncased("image"))]
    #[field(name = uncased("img-src"))]
    #[field(name = uncased("file"))]
    pub file: Vec<MaxLimitString<256>>,
}
impl ArtNew {
    pub fn search_condition(self) -> Option<Condition> {
        let mut condition = Condition::any();
        if self.file.len() == 0 {
            None
        } else {
            for filename in self.file {
                condition = condition.add(file_stores::Column::File.eq(filename.as_ref().as_str()));
            }
            Some(condition)
        }
    }
}

impl From<Vec<MaxLimitString<256>>> for ArtNew {
    fn from(s: Vec<MaxLimitString<256>>) -> Self {
        Self { file: s }
    }
}

pub struct ArtSaved {
    iid: i64,
    fid: i64,
}

impl ArtSaved {
    pub fn from_model(f: file_stores::Model, iid: i64) -> Self {
        Self {
            iid: iid,
            fid: f.id,
        }
    }
}

impl Into<illustrator_acts::ActiveModel> for ArtSaved {
    fn into(self) -> illustrator_acts::ActiveModel {
        illustrator_acts::ActiveModel {
            iid: Set(self.iid),
            fid: Set(self.fid),
            ..Default::default()
        }
    }
}
