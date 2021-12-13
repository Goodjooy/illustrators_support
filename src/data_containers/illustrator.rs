use sea_orm::{ActiveValue, Set, Value};
use serde::{Deserialize, Serialize};

use crate::{
    entity::{file_stores, illustrators, users},
    utils::data_structs::{MaxLimitString, RangeLimitString},
};

#[derive(Serialize, Deserialize, Clone)]
pub struct IllustratorOp {
    name: Option<RangeLimitString<1, 32>>,
    head: Option<MaxLimitString<256>>,
    home: Option<MaxLimitString<256>>,
    sponsor: Option<MaxLimitString<256>>,
}

fn set_value<V: Into<Value>, T, F>(src: &mut ActiveValue<V>, data: Option<T>, f: F)
where
    F: Fn(T) -> V,
{
    if let Some(data) = data {
        *src = Set::<V>(f(data))
    }
}

impl IllustratorOp {
    pub fn mix_with_model(self, model: illustrators::Model) -> illustrators::ActiveModel {
        let mut am: illustrators::ActiveModel = model.into();

        set_value(&mut am.name, self.name, |s| s.into());
        set_value(&mut am.head, self.head, |f| f.into());
        set_value(&mut am.home, self.home, |f| f.into());
        set_value(&mut am.sponsor, self.sponsor, |f| f.into());

        am
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct IllustratorNew {
    name: RangeLimitString<1, 32>,
    head: MaxLimitString<256>,
    home: MaxLimitString<256>,
    sponsor: MaxLimitString<256>,
}

impl Into<illustrators::ActiveModel> for IllustratorNew {
    fn into(self) -> illustrators::ActiveModel {
        illustrators::ActiveModel {
            name: Set(self.name.into()),
            head: Set(self.head.into()),
            home: Set(self.home.into()),
            sponsor: Set(self.sponsor.into()),
            ..Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct IllustratorTItle {
    iid: i64,
    name: String,
    head: String,
    home: String,
    sponsor: String,
    wconts: usize,
}

impl From<(illustrators::Model, usize)> for IllustratorTItle {
    fn from((ill, count): (illustrators::Model, usize)) -> Self {
        Self {
            iid: ill.id,
            name: ill.name,
            head: ill.head,
            home: ill.home,
            sponsor: ill.sponsor,
            wconts: count,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
struct ArtInfo {
    src: Option<String>,
    file: String,
}

impl From<file_stores::Model> for ArtInfo {
    fn from(f: file_stores::Model) -> Self {
        Self {
            src: f.src,
            file: f.file,
        }
    }
}
#[derive(Serialize, Deserialize, Clone)]
struct WantsInfo {
    name: String,
    qq: i64,
}

impl From<users::Model> for WantsInfo {
    fn from(f: users::Model) -> Self {
        Self {
            name: f.name,
            qq: f.qq,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Illustrator {
    iid: i64,
    name: String,
    head: String,
    home: String,
    sponser: String,
    arts: Vec<ArtInfo>,
    wants: Vec<WantsInfo>,
}

impl
    From<(
        illustrators::Model,
        Vec<file_stores::Model>,
        Vec<users::Model>,
    )> for Illustrator
{
    fn from(
        (ill, ill_arts, wants): (
            illustrators::Model,
            Vec<file_stores::Model>,
            Vec<users::Model>,
        ),
    ) -> Self {
        Self {
            iid: ill.id,
            name: ill.name,
            head: ill.head,
            home: ill.home,
            sponser: ill.sponsor,
            arts: ill_arts.into_iter().map(|art| art.into()).collect(),
            wants: wants.into_iter().map(|u| u.into()).collect(),
        }
    }
}
