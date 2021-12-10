use std::path::Path;

use rocket::{fs::NamedFile, http::Status, State};
use sea_orm::{ActiveModelTrait, ColumnTrait, Condition, EntityTrait, QueryFilter, Set};

use crate::{
    data_containers::{admin::Admin, r_result::RResult, users::UserLogin},
    database::Database,
    entity::file_stores,
    to_rresult,
    utils::{config::Config, multpart::MultPartFile},
};

crate::generate_controller!(
    FileServerController,
    "/images",
    user_file,
    upload
);

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
    file: MultPartFile<'_>,
    db: &State<Database>,
    config: &State<Config>,
) -> RResult<String> {
    let uid = to_rresult!(
        op,
        auth.id,
        Status::NonAuthoritativeInformation,
        "No User Id Found"
    );

    let file_save_path = Path::new(&config.consts.save_dir);
    let file_url_name = file.filename();

    let fs = file_stores::ActiveModel {
        uid: Set(uid),
        is_suit: Set(true as i8),
        file: Set(file.filename()),
        ..Default::default()
    };

    let _res = to_rresult!(rs, file.save_to(file_save_path).await);
    let _res = to_rresult!(rs, fs.insert(db.unwarp()).await);
    RResult::ok(file_url_name)
}
