use rocket::http::Cookie;
use serde::Serialize;

use crate::database::Database;

pub mod admin;
pub mod illustrator;
pub mod r_result;
pub mod users;

#[rocket::async_trait]
pub trait SelectBy<T> {
    async fn select_by(self, db: &Database) -> Result<Option<T>, sea_orm::DbErr>;
}

#[rocket::async_trait]
pub trait TryIntoWithDatabase<T> {
    type Error;
    async fn try_into(self, db: &Database) -> Result<T, Self::Error>;
}

pub trait IntoCookie {
    fn into_cookie(self, name: &str) -> Cookie;
}

impl<T: Serialize> IntoCookie for T {
    fn into_cookie(self, name: &str) -> Cookie {
        Cookie::new(
            name.to_string(),
            serde_json::to_string(&self).expect("Serialize Cookie Error"),
        )
    }
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
                    au.into_outcome(rocket::http::Status::Unauthorized)
                } else {
                    rocket::outcome::Outcome::Failure((rocket::http::Status::Unauthorized, "No auth info".to_string()))
                }
            }
        }
    };
}