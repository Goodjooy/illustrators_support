use sea_orm::Set;
use serde::{Deserialize, Serialize};

use crate::{
    entity::{illustrator_acts, illustrators, users},
    utils::{MaxLimitString, RangeLimitString},
};

#[derive(Serialize, Deserialize, Clone)]
pub struct IllustratorOp {
    name: Option<RangeLimitString<1, 32>>,
    head: Option<MaxLimitString<256>>,
    home: Option<MaxLimitString<256>>,
    sponsor: Option<MaxLimitString<256>>,
}

impl IllustratorOp {
    pub fn check_for_new(self) -> Option<IllustratorNew> {
        if self.name.is_some()
            && self.head.is_some()
            && self.home.is_some()
            && self.sponsor.is_some()
        {
            Some(IllustratorNew {
                name: self.name.unwrap(),
                head: self.head.unwrap(),
                home: self.home.unwrap(),
                sponsor: self.sponsor.unwrap(),
            })
        } else {
            None
        }
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
}

impl From<illustrators::Model> for IllustratorTItle {
    fn from(ill: illustrators::Model) -> Self {
        Self {
            iid: ill.id,
            name: ill.name,
            head: ill.head,
            home: ill.home,
            sponsor: ill.sponsor,
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
    arts: Vec<(String, bool)>,
    wants: Vec<(String, i64)>,
}

impl
    From<(
        illustrators::Model,
        Vec<illustrator_acts::Model>,
        Vec<users::Model>,
    )> for Illustrator
{
    fn from(
        (ill, ill_arts, wants): (
            illustrators::Model,
            Vec<illustrator_acts::Model>,
            Vec<users::Model>,
        ),
    ) -> Self {
        Self {
            iid: ill.id,
            name: ill.name,
            head: ill.head,
            home: ill.home,
            sponser: ill.sponsor,
            arts: ill_arts
                .into_iter()
                .map(|art| (art.pic, art.is_suit != 0))
                .collect(),
            wants: wants.into_iter().map(|u| (u.name, u.qq)).collect(),
        }
    }
}
