/**
 * @Author: Your name
 * @Date:   2021-12-10 18:12:44
 * @Last Modified by:   Your name
 * @Last Modified time: 2021-12-12 11:02:54
 */
use std::path::Path;

use sea_orm::{ActiveModelTrait, Set};

use crate::{
    database::Database,
    entity::file_stores,
    to_rresult,
    utils::{config::ConstConfig, multpart::MultPartFile, MaxLimitString},
};

use super::r_result::RResult;

#[derive(FromForm)]
pub struct FileUpload<'s> {
    //#[field(default = "Unknown")]
    src: Option<MaxLimitString<256>>,
    file: MultPartFile<'s>,
}
#[derive(FromForm)]
pub struct f<'v>{
    pub i :String,
    pub v:MultPartFile<'v>
}
#[derive(FromForm)]
pub struct FileUploads<'v>{
    pub data:Vec<MultPartFile<'v>>
}

impl FileUpload<'_> {
    pub async fn save(self, uid: i64, db: &Database, config: &ConstConfig) -> RResult<String> {
        let save_path = Path::new(&config.save_dir);
        let filename = self.file.filename();
        let fs = file_stores::ActiveModel {
            uid: Set(uid),
            file: Set(self.file.filename()),
            src: Set(self.src.clone().and_then(|s| Some(s.into()))),
            ..Default::default()
        };

        let _res = to_rresult!(rs, self.file.save_to(save_path).await);
        let _res = to_rresult!(rs, fs.insert(db.unwarp()).await);

        RResult::ok(filename)
    }
}
