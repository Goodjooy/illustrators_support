use std::{
    collections::HashMap,
    path::Path,
    str::FromStr,
    sync::{Mutex, RwLock},
};

use rocket::{fs::TempFile, serde::json::Json, State};
use sea_orm::{ActiveModelTrait, ColumnTrait, Condition, EntityTrait, QueryFilter, Set};
use uuid::Uuid;

use crate::{
    data_containers::{
        illustrator::{Illustrator, IllustratorNew, IllustratorTItle},
        r_result::RResult,
        users::UserLogin,
    },
    database::Database,
    entity::{illustrator_acts, illustrator_wants, illustrators, users},
    to_rresult,
};

const SAVE_PATH: &str = "./SAVES";

crate::generate_controller!(
    IllustratorController,
    "/illustrator",
    new_illustrator,
    add_art,
    illustrator_all,
    illustrator_detial
);

#[post("/new", data = "<input>")]
async fn new_illustrator(
    _auth: UserLogin,
    input: Json<IllustratorNew>,
    db: &State<Database>,
    ill_collect: &State<RwLock<HashMap<Uuid, i64>>>,
) -> RResult<Uuid> {
    let ill_new = (*input).clone();
    let ill_new: illustrators::ActiveModel = ill_new.into();

    let res = to_rresult!(
        rs,
        illustrators::Entity::insert(ill_new)
            .exec(db.unwarp())
            .await
    );

    let ident = uuid::Uuid::new_v4();
    let mut mut_ill = to_rresult!(rs, ill_collect.write());
    mut_ill.insert(ident.clone(), res.last_insert_id);

    RResult::ok(ident)
}
#[post("/add_arts/<ident>", data = "<file>")]
async fn add_art(
    _auth: UserLogin,
    ident: String,
    mut file: TempFile<'_>,
    db: &State<Database>,
    ill_collect: &State<Mutex<HashMap<Uuid, i64>>>,
) -> RResult<()> {
    if let Some(name) = file.name() {
        let ext = name
            .split(".")
            .last()
            .and_then(|s| Some(format!(".{}", s)))
            .unwrap_or_default();
        let new_name = Uuid::new_v4().to_string();
        let path = format!("{}{}", new_name, ext);

        let _res = to_rresult!(
            rs,
            file.move_copy_to(Path::new(SAVE_PATH).join(&path)).await
        );

        let ident = to_rresult!(rs, Uuid::from_str(&ident));
        let iid = {
            let map = to_rresult!(rs, ill_collect.lock());
            *to_rresult!(op, map.get(&ident), "Ident Not Found")
        };

        let iart = illustrator_acts::ActiveModel {
            iid: Set(iid),
            pic: Set(path),
            ..Default::default()
        };

        let _res = to_rresult!(rs, iart.insert(db.unwarp()).await);
        RResult::ok(())
    } else {
        RResult::err("file with out name")
    }
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
    let mut tc = Condition::any();
    for uid in wants {
        tc = tc.add(users::Column::Id.eq(uid.uid));
    }
    let wants = to_rresult!(rs, users::Entity::find().filter(tc).all(db.unwarp()).await);

    RResult::ok((ill_src, arts, wants).into())
}
