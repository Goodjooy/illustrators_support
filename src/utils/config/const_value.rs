use serde::Deserialize;

use super::ConfigTrait;

#[derive(Deserialize,Clone)]
pub struct ConstConfig {
    pub save_dir: String,
    //pub user_cookie: String,
    //pub admin_cookie: String,

}

impl Default for ConstConfig {
    fn default() -> Self {
        Self {
            save_dir: String::from("./SAVES"),
            //user_cookie: String::from("__uauth__"),
            //admin_cookie: String::from("__AD__VIRFF__"),
        }
    }
}

impl ConfigTrait for ConstConfig {
    
}