use lazy_static::__Deref;
use serde::Deserialize;
use std::ops::Deref;

use self::{authcation::AuthConfig, const_value::ConstConfig};

pub mod authcation;
pub mod const_value;
pub mod database;
pub mod invite_code;

#[derive(Deserialize, Default, Clone)]
pub struct Config {
    pub database: database::DbConfig,
    pub auth: AuthConfig,
    pub consts: ConstConfig,
    pub invite_codes: Option<invite_code::DefaultInviteCodeConfig>,
}

impl ConfigTrait for Config {}

pub enum RefConfig<'s, T: ConfigTrait> {
    Ref(&'s T),
    Owner(T),
}

impl<'s, T: ConfigTrait> Deref for RefConfig<'s, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            RefConfig::Ref(r) => *r,
            RefConfig::Owner(o) => o,
        }
    }
}

pub trait ConfigTrait {}
pub trait GetConfig: Sized + Send + Sync + ConfigTrait {
    fn get_config<'r>(config: &'r Option<Self>) -> RefConfig<'r, Self>;
    fn get_inner_config<'r>(config: Option<&'r Self>) -> RefConfig<'r, Self>;
}

impl<T: Sized + Send + Sync + ConfigTrait + Default> GetConfig for T {
    fn get_config<'r>(config: &'r Option<Self>) -> RefConfig<'r, Self> {
        match config {
            Some(s) => RefConfig::Ref(s),
            None => RefConfig::Owner(Default::default()),
        }
    }

    fn get_inner_config<'r>(config: Option<&'r Self>) -> RefConfig<'r, Self> {
        match config {
            Some(s) => RefConfig::Ref(s),
            None => RefConfig::Owner(Default::default()),
        }
    }
}
