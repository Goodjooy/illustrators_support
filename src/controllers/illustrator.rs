/**
 * @Author: Your name
 * @Date:   2021-12-01 20:56:53
 * @Last Modified by:   Your name
 * @Last Modified time: 2021-12-12 00:51:01
 */
use crate::{
    data_containers::{
        arts::{ArtNew, ArtSaved},
        illustrator::{Illustrator, IllustratorNew, IllustratorTItle},
        update_record::LastUpdate,
        users::UserLogin,
    },
    database::{
        update_bound::{TableName, UpdateRecordBound},
        Database,
    },
    entity::{file_stores, illustrator_acts, illustrator_wants, illustrators, users},
    to_rresult,
    utils::data_structs::{header_info::HeaderInfo, r_result::RResult, MaxLimitString},
};

use rocket::{http::Status, serde::json::Json, State};
use sea_orm::{
    sea_query::IntoCondition, ActiveModelTrait, ColumnTrait, Condition, EntityTrait, QueryFilter,
    QuerySelect, QueryTrait, RelationTrait, Set,
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
async fn illustrator_all(
    _auth: UserLogin,
    db: &State<Database>,
    record: HeaderInfo<'_, LastUpdate>,
) -> RResult<Vec<IllustratorTItle>> {
    let record = record.try_into().ok();
    let res = to_rresult!(
        rs,
        illustrators::Entity::find()
            // thouse code for record time bound
            .filter(
                Condition::all()
                    .add(Condition::all().ext_record_bound(
                        &illustrators::Column::Id,
                        TableName::ILLUSTRATORS,
                        record.clone()
                    ))
                    .add(Condition::all().ext_record_bound(
                        &illustrator_wants::Column::Id,
                        TableName::ILLUSTRATOR_WANTS,
                        record
                    ))
            )
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
    record: HeaderInfo<'_, LastUpdate>,
) -> RResult<Illustrator> {
    let record = record.try_into().ok();
    let (ill_src, arts) = to_rresult!(
        op,
        to_rresult!(
            rs,
            illustrators::Entity::find_by_id(id)
                .join_rev(
                    sea_orm::JoinType::LeftJoin,
                    illustrator_acts::Relation::Illustrators.def(),
                )
                .join(
                    sea_orm::JoinType::LeftJoin,
                    illustrator_acts::Relation::FileStores.def(),
                )
                .select_with(file_stores::Entity)
                .filter(Condition::all().ext_record_bound(
                    &file_stores::Column::Id,
                    TableName::FILE_STORES,
                    record.clone()
                ))
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
            .filter(
                illustrator_wants::Column::Iid
                    .eq(id)
                    .into_condition()
                    .ext_record_bound(
                        &illustrator_wants::Column::Id,
                        TableName::ILLUSTRATOR_WANTS,
                        record
                    )
            )
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
    let uid = to_rresult!(op, auth.id, "Bad Auth");
    if let Some(ill) = to_rresult!(
        rs,
        illustrators::Entity::find_by_id(id).one(db.unwarp()).await
    ) {
        let res = to_rresult!(
            rs,
            illustrator_wants::Entity::find()
                .filter(
                    sea_orm::Condition::all()
                        .add(illustrator_wants::Column::Uid.eq(uid))
                        .add(illustrator_wants::Column::Iid.eq(id))
                )
                .one(db.unwarp())
                .await
        );

        if let None = res {
            let want = illustrator_wants::ActiveModel {
                uid: Set(uid),
                iid: Set(ill.id),
                ..Default::default()
            };
            to_rresult!(rs, want.save(db.unwarp()).await);
            RResult::ok("添加想要成功".to_string())
        } else {
            RResult::err("你已经添加过了")
        }
    } else {
        RResult::err("目标画师不存在")
    }
}
