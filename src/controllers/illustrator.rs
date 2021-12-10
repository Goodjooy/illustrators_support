use std::{path::Path, time::Duration};

use crate::{
    data_containers::{
        arts::ArtNew,
        illustrator::{Illustrator, IllustratorNew, IllustratorTItle},
        r_result::RResult,
        users::UserLogin,
    },
    database::Database,
    entity::{illustrator_acts, illustrator_wants, illustrators, users},
    to_rresult,
    utils::{config::Config, lifetime_hashmap::LifeTimeHashMap},
};

use rocket::{form::Form, http::Status, serde::json::Json, State};
use sea_orm::{ActiveModelTrait, ColumnTrait, Condition, EntityTrait, QueryFilter, Set};

crate::generate_controller!(
    IllustratorController,
    "/illustrator",
    new_illustrator,
    add_art,
    illustrator_all,
    illustrator_detial,
    want_illustrator,
    // backen handler
    no_user_auth_post,
    no_user_auth_get
);

crate::no_auth_handle!(post, no_user_auth_post, User, "illustrator");
crate::no_auth_handle!(get, no_user_auth_get, User, "illustrator");

#[post("/new", data = "<input>")]
async fn new_illustrator(
    _auth: UserLogin,
    input: Json<serde_json::Value>,
    db: &State<Database>,
    ill_collect: &State<LifeTimeHashMap<String, i64>>,
) -> RResult<String> {
    let ill_new = to_rresult!(
        rs,
        super::into_entity::<IllustratorNew>(input),
        Status::UnprocessableEntity
    );
    let ill_new: illustrators::ActiveModel = ill_new.into();

    let res = to_rresult!(
        rs,
        illustrators::Entity::insert(ill_new)
            .exec(db.unwarp())
            .await
    );

    let ident = uuid::Uuid::new_v4();
    ill_collect.insert(
        ident.clone().to_string(),
        res.last_insert_id,
        Duration::from_secs(3600),
    );

    RResult::ok(ident.to_string())
}
#[post("/add_arts/<ident>", data = "<file>")]
async fn add_art<'s>(
    _auth: UserLogin,
    ident: String,
    file: rocket::form::Result<'s, Form<ArtNew<'s>>>,
    db: &State<Database>,
    config: &State<Config>,
    ill_collect: &State<LifeTimeHashMap<String, i64>>,
) -> RResult<&'static str> {
    let file = to_rresult!(
        rs,
        file,
        Status::UnprocessableEntity,
        "Form Struct Not Match"
    )
    .into_inner();

    let iid = to_rresult!(op, ill_collect.get(&ident), "Ident Not Found");

    let (f, arts) = file.into_save(iid);

    let iart: illustrator_acts::ActiveModel = arts.into();

    let _res = to_rresult!(rs, iart.insert(db.unwarp()).await);

    let _res = to_rresult!(rs, f.save_to(Path::new(&config.consts.save_dir)).await);

    RResult::ok("Upload File success")
}

#[get("/all")]
async fn illustrator_all(_auth: UserLogin, db: &State<Database>) -> RResult<Vec<IllustratorTItle>> {
    let res = to_rresult!(rs, illustrators::Entity::find().all(db.unwarp()).await);

    RResult::ok(res.into_iter().map(|i| i.into()).collect())
}

#[get("/<id>")]
async fn illustrator_detial(
    _auth: UserLogin,
    id: i64,
    db: &State<Database>,
) -> RResult<Illustrator> {
    let (ill_src, arts) = to_rresult!(
        op,
        to_rresult!(
            rs,
            illustrators::Entity::find_by_id(id)
                .find_with_related(illustrator_acts::Entity)
                .all(db.unwarp())
                .await
        )
        .into_iter()
        .next(),
        "not found illustrator"
    );

    let (_, wants) = to_rresult!(
        op,
        to_rresult!(
            rs,
            illustrators::Entity::find_by_id(id)
                .find_with_related(illustrator_wants::Entity)
                .all(db.unwarp())
                .await
        )
        .into_iter()
        .next(),
        "not found illustrator"
    );

    let wants = if wants.len() == 0 {
        vec![]
    } else {
        let mut tc = Condition::any();
        for uid in wants {
            tc = tc.add(users::Column::Id.eq(uid.uid));
        }
        to_rresult!(rs, users::Entity::find().filter(tc).all(db.unwarp()).await)
    };

    RResult::ok((ill_src, arts, wants).into())
}

#[post("/<id>")]
async fn want_illustrator(auth: UserLogin, id: i64, db: &State<Database>) -> RResult<String> {
    if let Some(ill) = to_rresult!(
        rs,
        illustrators::Entity::find_by_id(id).one(db.unwarp()).await
    ) {
        let want = illustrator_wants::ActiveModel {
            uid: Set(to_rresult!(op, auth.id, "Bad Auth")),
            iid: Set(ill.id),
            ..Default::default()
        };
        to_rresult!(rs, want.insert(db.unwarp()).await);
        RResult::ok("添加想要成功".to_string())
    } else {
        RResult::err("目标画师不存在")
    }
}
