use crate::data_containers::{IntoCookie, TryIntoWithDatabase};
use rocket::{
    http::{CookieJar, Status},
    serde::json::Json,
    State,
};
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

generate_controller!(
    UserController,
    "/user",
    user_login,
    user_new //backend handler
);

crate::from_cooke!(COOKIE_NAME, UserLogin);

#[post("/login", data = "<login_info>")]
async fn user_login(
    login_info: Json<serde_json::Value>,
    db: &State<Database>,
    cookies: &CookieJar<'_>,
) -> RResult<String> {
    let ulogin = to_rresult!(
        rs,
        super::into_entity::<UserLogin>(login_info),
        Status::UnprocessableEntity
    );
    let res = to_rresult!(
        op,
        to_rresult!(rs, ulogin.select_by(&*db).await),
        "User Name Or QQ Not Exist"
    );
    let uauth: UserLogin = res.into();
    let info = format!(
        "[ {} ] Login Success",
        match &(uauth.name) {
            Some(s) => s.as_ref().as_str(),
            None => "UNknowm",
        }
    );
    cookies.add_private(uauth.into_cookie(COOKIE_NAME));

    RResult::ok(info)
}

#[post("/new", data = "<input>")]
async fn user_new(input: Json<serde_json::Value>, db: &State<Database>) -> RResult<String> {
    let new_user = to_rresult!(
        rs,
        super::into_entity::<UserNew>(input),
        Status::UnprocessableEntity
    );
    let new_user: entity::users::ActiveModel =
        to_rresult!(rs, new_user.try_into_with_db(&*db).await);

    let _t = to_rresult!(rs, new_user.insert(db.unwarp()).await);

    RResult::ok(format!("Sign up success"))
}
