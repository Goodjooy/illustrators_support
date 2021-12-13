use crate::controllers::into_entity;
use crate::data_containers::invite::CodeSimple;
use rocket::http::Status;
use rocket::{serde::json::Json, State};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::data_containers::{
    admin::Admin, invite::InviteCodeNew, r_result::RResult, TryIntoWithDatabase,
};
use crate::entity::invites;
use crate::{to_rresult, Database};

#[post("/invite", data = "<input>")]
pub async fn add_invite_code(
    _auth: Admin,
    input: Json<serde_json::Value>,
    db: &State<Database>,
) -> RResult<&'static str> {
    let codes = to_rresult!(
        rs,
        into_entity::<InviteCodeNew>(input),
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

#[get("/invite/all")]
pub async fn get_all_invite_code(_auth: Admin, db: &State<Database>) -> RResult<Vec<CodeSimple>> {
    let res = to_rresult!(rs, invites::Entity::find().all(db.unwarp()).await);
    RResult::ok(res.into_iter().map(|c| c.into()).collect())
}
#[delete("/invite/<cid>")]
pub async fn remove_invite_code(
    _auth: Admin,
    cid: i64,
    db: &State<Database>,
) -> RResult<CodeSimple> {
    let res = to_rresult!(rs, invites::Entity::find_by_id(cid).one(db.unwarp()).await);
    if let Some(m) = res {
        let res = m.into();
        let _res = to_rresult!(
            rs,
            invites::Entity::delete_many()
                .filter(invites::Column::Id.eq(cid))
                .exec(db.unwarp())
                .await
        );
        RResult::ok(res)
    } else {
        RResult::status_err(
            Status::NotFound,
            format!("Target Invite code<id=={}> Not found", cid),
        )
    }
}
