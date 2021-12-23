use rocket::serde::json::Json;
use rocket::State;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};

use crate::{
    data_containers::{
        admin::Admin,
        file_store::{FileRemove, FileStatus},
    },
    database::Database,
    entity::{file_stores, illustrator_acts},
    to_rresult,
    utils::{config::Config, data_structs::r_result::RResult},
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
#[delete("/images", data = "<art_names>")]
pub async fn remove_arts(
    _auth: Admin,
    art_names: Json<FileRemove>,
    db: &State<Database>,
    config: &State<Config>,
) -> RResult<Vec<FileStatus>> {
    let mut result = Vec::with_capacity(art_names.files.len());
    for filename in art_names.into_inner().files {
        let res = to_rresult!(
            rs,
            file_stores::Entity::find()
                .filter(file_stores::Column::File.eq(filename))
                .one(db.unwarp())
                .await
        );

        match res {
            Some(f) => {
                let filepath = std::path::Path::new(&config.consts.save_dir).join(&f.file);
                //remove linked illurstrators
                let _r = to_rresult!(
                    rs,
                    illustrator_acts::Entity::delete_many()
                        .filter(illustrator_acts::Column::Fid.eq(f.id))
                        .exec(db.unwarp())
                        .await
                );
                //remove data in db
                let d: file_stores::ActiveModel = f.into();
                let _r = to_rresult!(rs, file_stores::Entity::delete(d).exec(db.unwarp()).await);

                // remove local file
                if filepath.exists() {
                    let _r = to_rresult!(rs, rocket::tokio::fs::remove_file(filepath).await);
                    result.push(FileStatus::Exist);
                } else {
                    result.push(FileStatus::NotInLocal);
                }
            }
            None => result.push(FileStatus::NotInDatabase),
        }
    }
    RResult::ok(result)
}
