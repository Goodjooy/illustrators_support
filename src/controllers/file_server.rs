use std::path::Path;

use rocket::{fs::NamedFile, State};
use sea_orm::{ColumnTrait, Condition, EntityTrait, QueryFilter};

use crate::{
    data_containers::{admin::Admin, users::UserLogin},
    database::Database,
    entity::illustrator_acts,
    utils::config::Config,
};

crate::generate_controller!(FileServerController, "/images", user_file, admin_file);

async fn load_file(
    file_name: String,
    db: &State<Database>,
    config: &State<Config>,
    is_admin: bool,
) -> Option<NamedFile> {
    let mut condition = Condition::all().add(illustrator_acts::Column::Pic.eq(file_name.as_str()));
    if !is_admin {
        condition = condition.add(illustrator_acts::Column::IsSuit.eq(true))
    }

    if let Some(_res) = illustrator_acts::Entity::find()
        .filter(condition)
        .one(db.unwarp())
        .await
        .ok()?
    {
        let path = Path::new(&config.consts.save_dir).join(&file_name);
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
    _auth: UserLogin,
    file_name: String,
    db: &State<Database>,
    config: &State<Config>,
) -> Option<NamedFile> {
    load_file(file_name, db, config, false).await
}
#[get("/<file_name>", rank = 1)]
async fn admin_file(
    _auth: Admin,
    file_name: String,
    db: &State<Database>,
    config: &State<Config>,
) -> Option<NamedFile> {
    load_file(file_name, db, config, true).await
}


#[test]
fn test_is_file(){
    let path=Path::new("M:/rust_project/illustrators_support/SAVES/6e10a45f-88ff-498c-b0a4-a76d8946d10a.jpeg");

    assert_eq!(path.is_file(),true);
}