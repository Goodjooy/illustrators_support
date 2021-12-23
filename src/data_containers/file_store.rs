/**
 * @Author: Your name
 * @Date:   2021-12-10 18:12:44
 * @Last Modified by:   Your name
 * @Last Modified time: 2021-12-13 00:33:05
 */
use std::path::Path;

use rocket::http::Status;
use sea_orm::{ActiveModelTrait, Set};
use serde::Deserialize;

use crate::{
    database::Database,
    entity::file_stores,
    to_rresult,
    utils::{
        config::const_value::ConstConfig,
        data_structs::{r_result::RResult, MaxLimitString},
        multpart::MultPartFile,
    },
};

#[derive(FromForm)]
pub struct FileUpload<'s> {
    //#[field(default = "Unknown")]
    src: Option<MaxLimitString<256>>,
    file: Option<MultPartFile<'s>>,
}
#[derive(FromForm)]
pub struct Files<'v> {
    pub files: Vec<FileUpload<'v>>,
}

impl FileUpload<'_> {
    pub async fn save(self, uid: i64, db: &Database, config: &ConstConfig) -> RResult<String> {
        if let Some(file) = self.file {
            let save_path = Path::new(&config.save_dir);
            let filename = file.filename();
            let fs = file_stores::ActiveModel {
                uid: Set(uid),
                file: Set(file.filename()),
                src: Set(self.src.clone().and_then(|s| Some(s.into()))),
                ..Default::default()
            };
            let _res = to_rresult!(rs, file.save_to(save_path).await);
            let _res = to_rresult!(rs, fs.insert(db.unwarp()).await);
            RResult::ok(filename)
        } else if let Some(src) = self.src {
            let src = src.into();
            let fs = file_stores::ActiveModel {
                uid: Set(uid),
                file: Set(src.clone()),
                src: Set(Some(src.clone())),
                ..Default::default()
            };
            let _res = to_rresult!(rs, fs.insert(db.unwarp()).await);
            RResult::ok(src)
        } else {
            RResult::status_err(Status::UnprocessableEntity, "Expect src but not provide")
        }
    }
}

#[derive(Deserialize)]
pub struct FileRemove{
    pub files:Vec<String>
}

#[derive(serde::Serialize)]
pub enum FileStatus {
    NotInDatabase,
    NotInLocal,
    Exist,
}