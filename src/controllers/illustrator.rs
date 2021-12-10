use crate::{
    data_containers::{
        arts::{ArtNew, ArtSaved},
        illustrator::{Illustrator, IllustratorNew, IllustratorTItle},
        r_result::RResult,
        users::UserLogin,
    },
    database::Database,
    entity::{file_stores, illustrator_acts, illustrator_wants, illustrators, users},
    to_rresult,
    utils::MaxLimitString,
};

use rocket::{http::Status, serde::json::Json, State};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, EntityTrait, QueryFilter, QuerySelect, RelationTrait,
    Set,
};

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
) -> RResult<i64> {
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

    RResult::ok(res.last_insert_id)
}
#[post("/add_arts/<iid>", data = "<file>")]
async fn add_art(
    _auth: UserLogin,
    iid: i64,
    file: Json<Vec<serde_json::Value>>,
    db: &State<Database>,
) -> RResult<&'static str> {
    let arc_news: ArtNew = file
        .into_inner()
        .into_iter()
        .map(|f| serde_json::from_value::<MaxLimitString<256>>(f).ok())
        .filter(|a| a.is_some())
        .map(|a| a.unwrap())
        .collect::<Vec<_>>()
        .into();

    if let Some(condition) = arc_news.search_condition() {
        let files = to_rresult!(
            rs,
            crate::entity::file_stores::Entity::find()
                .filter(condition)
                .all(db.unwarp())
                .await
        );
        let files = files
            .into_iter()
            .map(|f| ArtSaved::from_model(f, iid))
            .map(|f| -> illustrator_acts::ActiveModel { f.into() })
            .collect::<Vec<_>>();

        let _res = to_rresult!(
            rs,
            illustrator_acts::Entity::insert_many(files)
                .exec(db.unwarp())
                .await
        );

        RResult::ok("Upload File success")
    } else {
        RResult::status_err(Status::UnprocessableEntity, "No Match Struct Found")
    }
}

#[get("/all")]
async fn illustrator_all(_auth: UserLogin, db: &State<Database>) -> RResult<Vec<IllustratorTItle>> {
    let res = to_rresult!(
        rs,
        illustrators::Entity::find()
            .find_with_related(illustrator_wants::Entity)
            .all(db.unwarp())
            .await
    );

    RResult::ok(res.into_iter().map(|(m, c)| (m, c.len()).into()).collect())
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
                .join(
                    sea_orm::JoinType::InnerJoin,
                    illustrator_acts::Relation::Illustrators.def(),
                )
                .join(
                    sea_orm::JoinType::InnerJoin,
                    illustrator_acts::Relation::FileStores.def(),
                )
                .group_by(illustrators::Column::Id)
                .select_with(file_stores::Entity)
                .all(db.unwarp())
                .await
        )
        .into_iter()
        .next(),
        Status::NotFound,
        "Target illustrator Not Found"
    );

    let wants = to_rresult!(
        rs,
        illustrator_wants::Entity::find()
            .filter(illustrator_wants::Column::Iid.eq(id))
            .all(db.unwarp())
            .await
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
