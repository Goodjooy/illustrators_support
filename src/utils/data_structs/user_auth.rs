use std::time::Duration;

use rocket::{
    http::Status,
    request::{FromRequest, Outcome},
    State,
};

use crate::{
    data_containers::{self, admin::Admin, users::UserLogin},
};

use super::{crypto_string::CryptoString, lifetime_hashmap::LifeTimeMap, r_result::RResult};

pub enum Auth {
    User(Identify),
    Admin(Identify),
    Both { u: Identify, a: Identify },
}

struct Identify {
    id: i64,
    pwd: CryptoString<6, 16>,
}

impl From<data_containers::users::UserLogin> for Identify {
    fn from(src: data_containers::users::UserLogin) -> Self {
        let UserLogin {
            id,
            name: _,
            qq: _,
            password,
        }: UserLogin = src;
        Self {
            id: id.unwrap(),
            pwd: password,
        }
    }
}

impl From<data_containers::admin::Admin> for Identify {
    fn from(src: data_containers::admin::Admin) -> Self {
        let Admin {
            aid,
            name: _,
            password,
        } = src;
        Self {
            id: aid.unwrap(),
            pwd: password,
        }
    }
}

impl Identify {
    pub fn get_id(&self) -> i64 {
        self.id
    }
}

impl Auth {
    pub fn new_user(user: UserLogin) -> Self {
        Self::User(user.into())
    }
    pub fn new_admin(admin: Admin) -> Self {
        Self::Admin(admin.into())
    }

    pub fn expand_user(self, admin: Admin) -> Self {
        match self {
            Auth::User(u) => Self::Both { u, a: admin.into() },
            s => s,
        }
    }

    pub fn expand_admin(self, user: UserLogin) -> Self {
        match self {
            Auth::Admin(a) => Self::Both { u: user.into(), a },
            a => a,
        }
    }

    pub fn have_user_auth(&self) -> bool {
        match self {
            Self::User(_) | Self::Both { u: _, a: _ } => true,
            _ => false,
        }
    }

    pub fn have_admin_auth(&self) -> bool {
        match self {
            Self::Admin(_) | Self::Both { u: _, a: _ } => true,
            _ => false,
        }
    }

    pub fn get_uid(&self) -> Option<i64> {
        match self {
            Self::User(i) | Self::Both { u: i, a: _ } => Some(i.get_id()),
            _ => None,
        }
    }
    pub fn get_aid(&self) -> Option<i64> {
        match self {
            Self::Admin(i) | Self::Both { u: _, a: i } => Some(i.get_id()),
            _ => None,
        }
    }
}

pub trait Authable {
    type Inner;
}

impl Authable for UserLogin {
    type Inner = i64;
}

impl Authable for Admin {
    type Inner = i64;
}

pub struct Authentication<T: Authable>(pub T::Inner, pub String);

const HEADER_NAME: &str = "Authentication";
const LIVE_TIME: Duration = Duration::from_secs(7 * 24 * 60 * 60);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Authentication<UserLogin> {
    type Error = String;

    async fn from_request(
        request: &'r rocket::Request<'_>,
    ) -> rocket::request::Outcome<Self, Self::Error> {
        let map = request
            .guard::<&State<LifeTimeMap<String, Auth>>>()
            .await
            .unwrap();
        let seesion = match request.headers().get_one(HEADER_NAME) {
            Some(inner) => inner.to_string(),
            None => {
                return RResult::status_err(
                    Status::Unauthorized,
                    format!("User Auth Need | {}", HEADER_NAME),
                )
                .into_forword()
            }
        };
        let auth = match map.get_pop(&seesion) {
            Some(inner) => inner,
            None => {
                return RResult::status_err(Status::Unauthorized, format!("User Auth Death"))
                    .into_forword()
            }
        };

        let user_auth = match auth.get_uid() {
            Some(inner) => inner,
            None => {
                return RResult::status_err(Status::Unauthorized, format!("User Auth Need"))
                    .into_forword()
            }
        };

        map.insert(seesion.clone(), auth, LIVE_TIME.clone());

        Outcome::Success(Authentication(user_auth, seesion))
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Authentication<Admin> {
    type Error = String;

    async fn from_request(
        request: &'r rocket::Request<'_>,
    ) -> rocket::request::Outcome<Self, Self::Error> {
        let map = request
            .guard::<&State<LifeTimeMap<String, Auth>>>()
            .await
            .unwrap();
        let seesion = match request.headers().get_one(HEADER_NAME) {
            Some(inner) => inner.to_string(),
            None => {
                return RResult::status_err(
                    Status::Unauthorized,
                    format!("Admin Auth Need | {}", HEADER_NAME),
                )
                .into_forword()
            }
        };
        let auth = match map.get_pop(&seesion) {
            Some(inner) => inner,
            None => {
                return RResult::status_err(Status::Unauthorized, format!("Admin Auth Death"))
                    .into_forword()
            }
        };

        let admin_auth = match auth.get_aid() {
            Some(inner) => inner,
            None => {
                return RResult::status_err(Status::Unauthorized, format!("Admin Auth Need"))
                    .into_forword()
            }
        };

        map.insert(seesion.clone(), auth, LIVE_TIME.clone());

        Outcome::Success(Authentication(admin_auth, seesion))
    }
}


impl LifeTimeMap<String,Auth> {
    pub fn add_user_auth(&self,user:UserLogin){
        let session=uuid::Uuid::new_v4().to_string();
        let auth=Auth::new_user(user);
        self.insert(session, auth, LIVE_TIME);
    }

    pub fn add_admin_auth(&self,admin:Admin)->Option<Auth>{
        let session=uuid::Uuid::new_v4().to_string();
        let auth=Auth::new_admin(admin);
        self.insert(session, auth, LIVE_TIME)
    }

    pub fn expand_user(&self,key:String,admin:Admin)->Option<Auth>{
        let new=self.get_pop(&key)?.expand_user(admin);
        self.insert(key, new, LIVE_TIME)
    }

    pub fn expand_admin(&self,key:String,user:UserLogin)->Option<Auth>{
        let new=self.get_pop(&key)?.expand_admin(user);
        self.insert(key, new, LIVE_TIME)
    }
}