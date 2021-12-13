/**
 * @Author: Your name
 * @Date:   2021-12-12 10:32:55
 * @Last Modified by:   Your name
 * @Last Modified time: 2021-12-13 00:00:40
 */
use std::path::Path;

use rocket::{
    form::{Form, Result},
    fs::NamedFile,
    http::Status,
    State,
};
use sea_orm::{ColumnTrait, Condition, EntityTrait, QueryFilter};

use crate::{
    data_containers::{file_store::FileUpload, file_store::Files, users::UserLogin},
    database::Database,
    entity::file_stores,
    to_rresult,
    utils::{config::Config, data_structs::r_result::RResult},
};

crate::generate_controller!(FileServerController, "/images", user_file, upload, uploads);

async fn load_file(
    file_name: String,
    db: &State<Database>,
    config: &State<Config>,
    is_admin: bool,
) -> Option<NamedFile> {
    let mut condition = Condition::all().add(file_stores::Column::File.eq(file_name.as_str()));
    if !is_admin {
        condition = condition.add(file_stores::Column::IsSuit.eq(true))
    }

    if let Some(res) = file_stores::Entity::find()
        .filter(condition)
        .one(db.unwarp())
        .await
        .ok()?
    {
        let path = Path::new(&config.consts.save_dir).join(&res.file);
        if path.is_file() {
            NamedFile::open(path).await.ok()
        } else {
            None
        }
    } else {
        None
    }
}
#[get("/<file_name>", rank = 2)]
async fn user_file(
    file_name: String,
    db: &State<Database>,
    config: &State<Config>,
) -> Option<NamedFile> {
    load_file(file_name, db, config, true).await
}

#[post("/upload", data = "<file>")]
async fn upload(
    auth: UserLogin,
    file: Result<'_, Form<FileUpload<'_>>>,
    db: &State<Database>,
    config: &State<Config>,
) -> RResult<String> {
    let uid = to_rresult!(
        op,
        auth.id,
        Status::NonAuthoritativeInformation,
        "No User Id Found"
    );
    let fu = to_rresult!(rs, file, Status::UnprocessableEntity).into_inner();

    fu.save(uid, &db, &config.consts).await
}

#[post("/uploads", data = "<file>")]
async fn uploads(
    auth: UserLogin,
    file: Result<'_, Form<Files<'_>>>,
    db: &State<Database>,
    config: &State<Config>,
) -> RResult<Vec<RResult<String>>> {
    let uid = to_rresult!(
        op,
        auth.id,
        Status::NonAuthoritativeInformation,
        "No User Id Found"
    );
    let fu = to_rresult!(rs, file, Status::UnprocessableEntity).into_inner();
    log::info!("file counts {}", fu.files.len());
    let mut res = Vec::with_capacity(fu.files.len());
    for f in fu.files {
        res.push(f.save(uid, &db, &config.consts).await);
        //res.push(RResult::ok(format!("{}",f.filename())))
    }
    log::info!("{:?}", res);
    RResult::ok(res)
}
