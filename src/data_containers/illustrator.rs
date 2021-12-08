use sea_orm::Set;
use serde::{Deserialize, Serialize};

use crate::{
    entity::{illustrator_acts, illustrators, users},
    utils::{MaxLimitString, RangeLimitString},
};

#[derive(Serialize, Deserialize, Clone)]
pub struct IllustratorNew {
    name: RangeLimitString<1, 32>,
    home: MaxLimitString<256>,
}

impl Into<illustrators::ActiveModel> for IllustratorNew {
    fn into(self) -> illustrators::ActiveModel {
        illustrators::ActiveModel {
            name: Set(self.name.into()),
            home: Set(self.home.into()),
            ..Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct IllustratorTItle {
    iid: i64,
    name: String,
    home: String,
}

impl From<illustrators::Model> for IllustratorTItle {
    fn from(ill: illustrators::Model) -> Self {
        Self {
            iid: ill.id,
            name: ill.name,
            home: ill.home,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Illustrator {
    iid: i64,
    name: String,
    home: String,
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
            home: ill.home,
            arts: ill_arts
                .into_iter()
                .map(|art| (art.pic, art.is_suit != 0))
                .collect(),
            wants: wants.into_iter().map(|u| (u.name, u.qq)).collect(),
        }
    }
}
