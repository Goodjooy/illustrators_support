use serde::Deserialize;

use crate::utils::RangeLimitString;

use super::ConfigTrait;

#[derive(Deserialize,Clone)]
pub struct DefaultInviteCodeConfig{
    pub codes:Vec<RangeLimitString<8,36>>
}

impl ConfigTrait for DefaultInviteCodeConfig {
    
}