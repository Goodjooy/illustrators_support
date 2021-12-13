use serde::Deserialize;

use super::ConfigTrait;

#[derive(Deserialize, Clone)]
pub struct AuthConfig {
    pub super_admin_auth: String,
    // pub invite_code: String,
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            super_admin_auth: "SUPER_ADMIN_PASSWORD".to_string(),
            // invite_code: "INIT_INVITE_CODE".to_string(),
        }
    }
}

impl ConfigTrait for AuthConfig {}
