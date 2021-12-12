/**
 * @Author: Your name
 * @Date:   2021-12-10 18:12:44
 * @Last Modified by:   Your name
 * @Last Modified time: 2021-12-13 00:33:05
 */
use std::path::Path;

use rocket::{
    data::{ByteUnit, DataStream, FromData},
    form::FromFormField,
    fs::FileServer,
    Data,
};
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
pub struct Files<'v> {
    pub files: Vec<MultPartFile<'v>>,
}

#[async_trait]
impl<'r> rocket::form::FromForm<'r> for Files<'r> {
    type Context = (bool, Self, bool);

    fn init(opts: rocket::form::Options) -> Self::Context {
        (false, Self { files: Vec::new() }, opts.strict)
    }

    fn push_value(ctxt: &mut Self::Context, field: rocket::form::ValueField<'r>) {
        log::info!("Get plain Value {} {}", field.name, field.value);
        if ctxt.2 == true {
            ctxt.0 = true
        }
    }

    async fn push_data(ctxt: &mut Self::Context, field: rocket::form::DataField<'r, '_>) {
        log::info!(
            "loading form field data {} | {:?} | {}",
            &field.content_type,
            &field.file_name,
            &field.name.as_name()
        );

        let (bork, data, strick) = ctxt;
        if field.name.as_name().as_str() == "files" {
            let res = match <MultPartFile as FromFormField>::from_data(field).await {
                Ok(file) => {
                    log::info!("accept file success! {}", file.filename());
                    file
                }
                Err(err) => {
                    log::error!("load file failure {}", err);
                    return;
                }
            };
            data.files.push(res)
        } else {
            log::warn!("Unknow form name [{}]", field.name.as_name());
            if strick == &true {
                *bork = true;
            }
        }
    }

    fn finalize(ctxt: Self::Context) -> rocket::form::Result<'r, Self> {
        let (bok, data, _) = ctxt;
        if bok {
            Err(rocket::form::Error::validation(
                "strict Mod with extra info",
            ))?
        }
        log::info!("load size: {}", data.files.len());

        Ok(data)
    }
}

impl<'r> FromData<'r> for FileUpload<'r> {
    type Error = String;

    fn from_data<'life0, 'async_trait>(
        req: &'r rocket::Request<'life0>,
        data: Data<'r>,
    ) -> core::pin::Pin<
        Box<
            dyn core::future::Future<Output = rocket::data::Outcome<'r, Self>>
                + core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'r: 'async_trait,
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        todo!()
    }
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
