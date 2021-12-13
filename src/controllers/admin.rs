use crate::data_containers::{IntoCookie, SelectBy, TryIntoWithConfig};

use crate::utils::config::Config;
use crate::utils::data_structs::r_result::RResult;
use crate::{data_containers::admin::Admin, database::Database};

use rocket::http::Status;
use rocket::{http::CookieJar, serde::json::Json, State};
use sea_orm::ActiveModelTrait;

use crate::{data_containers::admin::AdminNew, entity::admins, to_rresult};

mod illustrator;
mod invite_code;
mod serve_image;
mod user;

const ADMIN_COOKIE_NAME: &str = "Admin-Auth";

crate::generate_controller!(
    AdminController,
    "/admin",
    new_admin,
    admin_login,
    // invite code
    invite_code::add_invite_code,
    invite_code::get_all_invite_code,
    invite_code::remove_invite_code,
    // serve image
    serve_image::make_art_suti,
    // illustrator
    illustrator::edit,
    // back handler
    no_user_auth_post,
    no_user_auth_get
);
crate::from_cooke!(ADMIN_COOKIE_NAME, Admin);

crate::no_auth_handle!(post, no_user_auth_post, Admin, "admin");
crate::no_auth_handle!(get, no_user_auth_get, Admin, "admin");

#[post("/new", data = "<input>")]
async fn new_admin(
    input: Json<serde_json::Value>,
    db: &State<Database>,
    config: &State<Config>,
) -> RResult<&'static str> {
    let admin_info = to_rresult!(
        rs,
        super::into_entity::<AdminNew>(input),
        Status::UnprocessableEntity
    );
    let save_mod: admins::ActiveModel = to_rresult!(rs, admin_info.try_into_with_config(&*config));

    let _res = to_rresult!(rs, save_mod.insert(db.unwarp()).await);
    RResult::ok("Create new admin success")
}

#[post("/login", data = "<input>")]
async fn admin_login(
    input: Json<serde_json::Value>,
    db: &State<Database>,
    cookes: &CookieJar<'_>,
) -> RResult<&'static str> {
    let admin = to_rresult!(
        rs,
        super::into_entity::<Admin>(input),
        Status::UnprocessableEntity
    );
    let model = to_rresult!(
        op,
        to_rresult!(rs, admin.select_by(&*db).await),
        "Admin Info Not Found"
    );
    let admin: Admin = model.into();
    let cook = admin.into_cookie(ADMIN_COOKIE_NAME);
    cookes.add_private(cook);

    RResult::ok("Login success")
}
