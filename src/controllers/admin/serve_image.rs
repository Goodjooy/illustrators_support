use rocket::State;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};

use crate::{
    data_containers::{admin::Admin, r_result::RResult},
    database::Database,
    to_rresult,
};

#[post("/images/<art_name>")]
pub async fn make_art_suti(
    _auth: Admin,
    art_name: String,
    db: &State<Database>,
) -> RResult<&'static str> {
    match to_rresult!(
        rs,
        crate::entity::file_stores::Entity::find()
            .filter(crate::entity::file_stores::Column::File.eq(art_name))
            .one(db.unwarp())
            .await
    ) {
        Some(res) => {
            let mut active: crate::entity::file_stores::ActiveModel = res.into();
            active.is_suit = Set(true as i8);
            let _r = to_rresult!(rs, active.update(db.unwarp()).await);
            RResult::ok("Switch TO suit Sucess")
        }
        _ => RResult::err("No Such File"),
    }
}

