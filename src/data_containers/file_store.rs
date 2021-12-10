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
    src: Option<MaxLimitString<256>>,
    file: MultPartFile<'s>,
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
