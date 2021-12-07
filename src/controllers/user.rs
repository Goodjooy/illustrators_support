use crate::{data_containers::TryIntoWithConfig, utils::config::Config};
use rocket::{http::CookieJar, serde::json::Json, State};
use sea_orm::ActiveModelTrait;

use crate::{
    data_containers::{
        r_result::RResult,
        users::{UserLogin, UserNew},
        SelectBy,
    },
    database::Database,
    entity, generate_controller, to_rresult,
};

const COOKIE_NAME: &str = "__uauth__";

generate_controller!(UserController, "/user", user_login, user_new);

crate::from_cooke!(COOKIE_NAME, UserLogin);

#[post("/login", data = "<login_info>")]
async fn user_login(
    login_info: Json<UserLogin>,
    db: &State<Database>,
    cookies: &CookieJar<'_>,
) -> RResult<String> {
    let ulogin = (*login_info).clone();
    let res = to_rresult!(
        op,
        to_rresult!(rs, ulogin.select_by(&*db).await),
        "User Name Or QQ Not Exist"
    );
    let uauth: UserLogin = res.into();
    let info = format!("[ {} ] Login Success", uauth.name);
    cookies.add_private(uauth.to_cookie(COOKIE_NAME));

    RResult::ok(info)
}
#[post("/new", data = "<input>")]
async fn user_new(
    input: Json<UserNew>,
    db: &State<Database>,
    config: &State<Config>,
) -> RResult<String> {
    let new_user = (*input).clone();
    let new_user: entity::users::ActiveModel =
        to_rresult!(rs, new_user.try_into_with_config(&*config));

    let _t = to_rresult!(rs, new_user.insert(db.unwarp()).await);

    RResult::ok(format!("Sign up success"))
}
