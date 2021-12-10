use std::path::Path;

use rocket::{
    form::{Form, Result},
    fs::NamedFile,
    http::Status,
    State,
};
use sea_orm::{ColumnTrait, Condition, EntityTrait, QueryFilter};

use crate::{
    data_containers::{file_store::FileUpload, r_result::RResult, users::UserLogin},
    database::Database,
    entity::file_stores,
    to_rresult,
    utils::config::Config,
};

crate::generate_controller!(FileServerController, "/images", user_file, upload);

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
