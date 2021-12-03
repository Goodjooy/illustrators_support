use crate::data_containers::TryIntoWithDatabase;
use rocket::{
    http::{CookieJar, Status},
    request::{FromRequest, Outcome},
    serde::json::Json, State,
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

generate_controller!(UserController, "/user", user_login, user_new);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserLogin {
    async fn from_request(request: &'r rocket::Request<'_>) -> Outcome<Self, Self::Error> {
        let jar = request.cookies();
        if let Some(cookie) = jar.get_private(COOKIE_NAME) {
            let value=cookie.value();
            let au = RResult::from_result(serde_json::from_str::<UserLogin>(value));
            au.into_outcome(Status::Unauthorized)
        } else {
            Outcome::Failure((Status::Unauthorized, "No auth info".to_string()))
        }
    }
    type Error = String;
}

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
async fn user_new(input: Json<UserNew>, db: &State<Database>) -> RResult<String> {
    let new_user = (*input).clone();
    let new_user: entity::users::ActiveModel =
        to_rresult!(rs, TryIntoWithDatabase::try_into(new_user, &*db).await);

    let _t = to_rresult!(rs, new_user.insert(db.unwarp()).await);

    RResult::ok(format!("Sign up success"))
}
