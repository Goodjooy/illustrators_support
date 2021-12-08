use crate::data_containers::{IntoCookie, SelectBy, TryIntoWithConfig};

use crate::utils::config::Config;
use crate::{data_containers::admin::Admin, database::Database};

use rocket::{http::CookieJar, serde::json::Json, State};
use sea_orm::ActiveModelTrait;

use crate::{
    data_containers::{admin::AdminNew, r_result::RResult},
    entity::admins,
    to_rresult,
};

mod opearte;
mod views;

const __ADMIN_COOKIE_NAME__: &str = "__AD__VIRFF__";

crate::generate_controller!(
    AdminController,
    "/admin",
    new_admin,
    admin_login,
    opearte::add_invite_code,
    opearte::make_art_suti
);
crate::from_cooke!(__ADMIN_COOKIE_NAME__, Admin);

#[post("/new", data = "<input>")]
async fn new_admin(
    input: Json<AdminNew>,
    db: &State<Database>,
    config: &State<Config>,
) -> RResult<&'static str> {
    let admin_info = (*input).clone();
    let save_mod: admins::ActiveModel = to_rresult!(rs, admin_info.try_into_with_config(&*config));

    let _res = to_rresult!(rs, save_mod.insert(db.unwarp()).await);
    RResult::ok("Create new admin success")
}
#[post("/login", data = "<input>")]
async fn admin_login(
    input: Json<Admin>,
    db: &State<Database>,
    cookes: &CookieJar<'_>,
) -> RResult<&'static str> {
    let admin = (*input).clone();
    let model = to_rresult!(
        op,
        to_rresult!(rs, admin.select_by(&*db).await),
        "Admin Info Not Found"
    );
    let admin: Admin = model.into();
    let cook = admin.into_cookie(__ADMIN_COOKIE_NAME__);
    cookes.add_private(cook);

    RResult::ok("Login success")
}
