use crate::{
    data_containers::{
        admin::Admin, invite::InviteCodeNew, r_result::RResult, TryIntoWithDatabase,
    },
    database::Database,
    entity::{illustrator_acts, invites},
    to_rresult,
};
use rocket::{http::Status, serde::json::Json, State};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};

#[post("/invite", data = "<input>")]
pub async fn add_invite_code(
    _auth: Admin,
    input: Json<serde_json::Value>,
    db: &State<Database>,
) -> RResult<&'static str> {
    let codes = to_rresult!(
        rs,
        super::super::into_entity::<InviteCodeNew>(input),
        Status::UnprocessableEntity
    );

    let codes: Vec<invites::ActiveModel> =
        to_rresult!(rs, TryIntoWithDatabase::try_into_with_db(codes, &*db).await);

    let _res = to_rresult!(
        rs,
        invites::Entity::insert_many(codes).exec(db.unwarp()).await
    );

    RResult::ok("Add invite code success")
}

#[post("/audit/<art_name>")]
pub async fn make_art_suti(
    _auth: Admin,
    art_name: String,
    db: &State<Database>,
) -> RResult<&'static str> {
    if let Some(res) = to_rresult!(
        rs,
        illustrator_acts::Entity::find()
            .filter(illustrator_acts::Column::Pic.eq(art_name))
            .one(db.unwarp())
            .await
    ) {
        let mut active: illustrator_acts::ActiveModel = res.into();
        active.is_suit = Set(true as i8);
        let _r = to_rresult!(rs, active.update(db.unwarp()).await);
        RResult::ok("Switch TO SUIT Sucess")
    } else {
        RResult::err("No Such File")
    }
}
