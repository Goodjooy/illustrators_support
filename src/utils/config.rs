use serde::Deserialize;

#[derive(Deserialize, Default,Clone)]
pub struct Config {
    pub database: DbConfig,
    pub auth: AuthConfig,
    pub consts: ConstConfig,
}

#[derive(Deserialize,Clone)]
pub struct DbConfig {
    pub url: String,
    pub max_conn: u32,
    pub min_conn: u32,
}

impl Default for DbConfig {
    fn default() -> Self {
        Self {
            url: "".to_string(),
            max_conn: 8,
            min_conn: 0,
        }
    }
}

#[derive(Deserialize,Clone)]
pub struct AuthConfig {
    pub super_admin_auth: String,
    pub invite_code: String,
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            super_admin_auth: "SUPER_ADMIN_PASSWORD".to_string(),
            invite_code: "INIT_INVITE_CODE".to_string(),
        }
    }
}
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
