use std::path::Path;

use rocket::{
    data::{Capped, DataStream, FromData, Outcome, ToByteUnit},
    form::FromFormField,
    http::Status,
    tokio::fs::File,
    Data,
};
use uuid::Uuid;
pub struct MultPartFile<'r> {
    data: DataStream<'r>,
    filename: String,
    file_ext: &'r str,
}

impl MultPartFile<'_> {
    fn file_type_match(head: &[u8]) -> Option<&'static str> {
        Some(match head[0..2] {
            [0xff, 0xd8] => "jpeg",
            [0x42, 0x4d] => "bmp",
            [0x4d, 0x4d] | [0x49, 0x48] => "tiff",
            _ => match head[0..5] {
                [0x00, 0x00, 0x02, 0x00, 0x00] | [0x00, 0x00, 0x10, 0x00, 0x00] => "tga",
                _ => match head[0..6] {
                    [0x47, 0x49, 0x46, 0x38, 0x39, 0x61] | [0x47, 0x49, 0x46, 0x38, 0x37, 0x61] => {
                        "gif"
                    }
                    _ => match head[0..8] {
                        [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A] => "png",
                        [0x00, 0x00, 0x01, 0x00, 0x01, 0x00, 0x20, 0x20] => "ico",
                        [0x00, 0x00, 0x02, 0x00, 0x01, 0x00, 0x20, 0x20] => "cur",
                        _ => return None,
                    },
                },
            },
        })
    }
}

#[rocket::async_trait]
impl<'r> FromData<'r> for MultPartFile<'r> {
    type Error = &'static str;

    async fn from_data(
        req: &'r rocket::Request<'_>,
        mut data: Data<'r>,
    ) -> rocket::data::Outcome<'r, Self> {
        let limit = req.limits().get("multpart").unwrap_or(2u32.mebibytes());
        let f_head = data.peek(8).await;
        let ext = if let Some(ext) = Self::file_type_match(f_head) {
            Some(ext)
        } else if let Some(f_type) = req
            .headers()
            .get("Content-Type")
            .filter(|s| s.starts_with("image"))
            .next()
        {
            if let Some(idx) = f_type.find("/") {
                Some(&f_type[idx + 1..])
            } else {
                None
            }
        } else {
            None
        };

        if let Some(ext) = ext {
            let res = Self {
                data: data.open(limit),
                filename: Uuid::new_v4().to_string(),
                file_ext: ext,
            };
            rocket::data::Outcome::Success(res)
        } else {
            rocket::data::Outcome::Failure((Status::NotAcceptable, "File Type Can not Detect"))
        }
    }
}
#[rocket::async_trait]
impl<'v> FromFormField<'v> for MultPartFile<'v> {
    async fn from_data(field: rocket::form::DataField<'v, '_>) -> rocket::form::Result<'v, Self> {
        let res = <Self as FromData>::from_data(field.request, field.data).await;
        let res = Outcome::success_or(
            res,
            rocket::form::Error::validation("File Type Cannot Identify"),
        );
        Ok(res?)
    }
}

impl MultPartFile<'_> {
    pub async fn save_to<P: AsRef<Path>>(self, path: P) -> std::io::Result<Capped<File>> {
        let res = self.data.into_file(path.as_ref().join(self.filename)).await;
        res
    }
    pub fn filename(&self) -> String {
        format!("{}.{}",self.filename,self.file_ext())
    }
    pub fn file_ext(&self) -> &str {
        &self.file_ext
    }
}
