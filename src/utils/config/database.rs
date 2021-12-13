use serde::Deserialize;

use super::ConfigTrait;

#[derive(Deserialize,Clone)]
pub struct DbConfig {
    pub url: String,
    pub db_log:bool,
    pub max_conn: u32,
    pub min_conn: u32,
}

impl Default for DbConfig {
    fn default() -> Self {
        Self {
            url: "".to_string(),
            max_conn: 8,
            min_conn: 0,
            db_log: true,
        }
    }
}

impl ConfigTrait for DbConfig {
    
}