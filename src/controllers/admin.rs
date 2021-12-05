use crate::{data_containers::TryIntoWithDatabase, database::Database};
use rocket::{serde::json::Json, State};
use sea_orm::ActiveModelTrait;

use crate::{
    data_containers::{admin::AdminNew, r_result::RResult},
    entity::admins,
    to_rresult,
};

crate::generate_controller!(AdminController, "/admin", new_admin);

#[post("/new", data = "<input>")]
async fn new_admin(input: Json<AdminNew>, db: &State<Database>) -> RResult<&'static str> {
    let admin_info = (*input).clone();
    let save_mod: admins::ActiveModel =
        to_rresult!(rs, TryIntoWithDatabase::try_into(admin_info, &*db).await);

    let _res = to_rresult!(rs, save_mod.insert(db.unwarp()).await);
    RResult::ok("Create new admin success")
}
