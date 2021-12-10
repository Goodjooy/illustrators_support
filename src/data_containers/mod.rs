use rocket::http::Cookie;
use serde::Serialize;

use crate::{database::Database, utils::config::Config};

pub mod admin;
pub mod illustrator;
pub mod invite;
pub mod r_result;
pub mod users;
pub mod arts;

#[rocket::async_trait]
pub trait SelectBy<T> {
    async fn select_by(self, db: &Database) -> Result<Option<T>, sea_orm::DbErr>;
}

#[rocket::async_trait]
pub trait TryIntoWithDatabase<T> {
    type Error;
    async fn try_into_with_db(self, db: &Database) -> Result<T, Self::Error>;
}

pub trait IntoCookie {
    fn into_cookie(self, name: &str) -> Cookie;
}

impl<T: Serialize> IntoCookie for T {
    fn into_cookie(self, name: &str) -> Cookie {
        Cookie::build(
            name.to_string(),
            serde_json::to_string(&self).expect("Serialize Cookie Error"),
        )
        .same_site(rocket::http::SameSite::None)
        .http_only(false)
        .secure(true)
        .finish()
    }
}

pub trait TryIntoWithConfig<T> {
    type Error;
    fn try_into_with_config(self, config: &Config) -> Result<T, Self::Error>;
}

#[macro_export]
macro_rules! from_cooke {
    ($cm:ident,$tg:ident) => {
        #[rocket::async_trait]
        impl<'r> rocket::request::FromRequest<'r> for $tg {
            type Error = String;

            async fn from_request(
                request: &'r rocket::Request<'_>,
            ) -> rocket::request::Outcome<Self, Self::Error> {
                let jar = request.cookies();
                if let Some(cookie) = jar.get_private($cm) {
                    let value = cookie.value();
                    let au = RResult::from_result(serde_json::from_str::<$tg>(value));
                    au.into_forword()
                } else if let Some(cookie) = jar.get_private_pending($cm) {
                    let value = cookie.value();
                    let au = RResult::from_result(serde_json::from_str::<$tg>(value));
                    au.into_forword()
                } else {
                    rocket::outcome::Outcome::Forward(())
                }
            }
        }
    };
}
