use rocket::{
    data::{FromData, ToByteUnit},
    form::FromFormField,
    http::Status,
    tokio::io::AsyncWriteExt,
};
use std::path::Path;

use rocket::{data::ByteUnit, http::ContentType, tokio::fs::File, Data};
use uuid::Uuid;
pub struct MultPartFile<'r> {
    data: Vec<u8>,
    filename: String,
    file_ext: &'r str,
}

impl MultPartFile<'_> {
    fn content_type_match(ftype: &ContentType) -> Option<&'static str> {
        Some(match ftype {
            f if f.is_aac() => "acc",
            f if f.is_avif() => "avif",
            f if f.is_bmp() => "bmp",
            f if f.is_binary() => "bin",
            f if f.is_bytes() => "bin",
            f if f.is_css() => "css",
            f if f.is_csv() => "csv",
            f if f.is_flac() => "flac",
            f if f.is_gif() => "gif",
            f if f.is_gzip() => "gz",
            f if f.is_html() => "html",
            f if f.is_ical() => "ics",
            f if f.is_icon() => "ico",
            f if f.is_javascript() => "js",
            f if f.is_jpeg() => "jpeg",
            f if f.is_json() => "json",
            f if f.is_mov() => "mov",
            f if f.is_mp4() => "mp4",
            f if f.is_mpeg() => "mpg",
            f if f.is_msgpack() => "msgpack",
            f if f.is_ogg() => "ogg",
            f if f.is_otf() => "otf",
            f if f.is_pdf() => "pdf",
            f if f.is_png() => "png",
            f if f.is_svg() => "svg",
            f if f.is_tar() => "tar",
            f if f.is_text() => "txt",
            f if f.is_ttf() => "ttf",
            f if f.is_tiff() => "tiff",
            f if f.is_wasm() => "wasm",
            f if f.is_wav() => "wav",
            f if f.is_weba() => "weba",
            f if f.is_webm() => "webm",
            f if f.is_webp() => "webp",
            f if f.is_woff() => "woff",
            f if f.is_woff2() => "woff2",
            f if f.is_xml() => "xml",
            f if f.is_zip() => "zip",
            _ => return None,
        })
    }
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

    async fn load_from_data<'r>(
        content_type: Option<&ContentType>,
        mut data: Data<'r>,
        limit: ByteUnit,
    ) -> Option<MultPartFile<'r>> {
        let fhead = data.peek(8).await;
        let file_ext = if let Some(ftype) = content_type.and_then(|c| Self::content_type_match(c)) {
            ftype
        } else if let Some(ext) = Self::file_type_match(fhead) {
            ext
        } else {
            return None;
        };
        let innerdata = data.open(limit);
        let data = innerdata.into_bytes().await.ok()?.into_inner();
        let res = MultPartFile::<'r> {
            data,
            filename: Uuid::new_v4().to_string(),
            file_ext: file_ext,
        };

        Some(res)
    }
}

#[rocket::async_trait]
impl<'r> FromData<'r> for MultPartFile<'r> {
    type Error = &'static str;

    async fn from_data(
        req: &'r rocket::Request<'_>,
        data: Data<'r>,
    ) -> rocket::data::Outcome<'r, Self> {
        let limit = req.limits().get("multpart").unwrap_or(2u32.mebibytes());
        let ftype = req.content_type();

        match Self::load_from_data(ftype, data, limit).await {
            Some(ext) => rocket::data::Outcome::Success(ext),
            _ => rocket::data::Outcome::Failure((
                Status::NotAcceptable,
                "File Type Can not Identify",
            )),
        }
    }
}
#[rocket::async_trait]
impl<'v> FromFormField<'v> for MultPartFile<'v> {
    async fn from_data(field: rocket::form::DataField<'v, '_>) -> rocket::form::Result<'v, Self> {
        log::info!(
            "loading form field data {} | {:?} | {}",
            &field.content_type,
            &field.file_name,
            &field.name.as_name()
        );

        let limlt = field
            .request
            .limits()
            .get("multpart")
            .unwrap_or(ByteUnit::Mebibyte(2));
        let res = Self::load_from_data(Some(&field.content_type), field.data, limlt).await;
        let res = match res {
            Some(s) => Ok(s),
            None => Err(rocket::form::Error::validation(
                "File Type Can not Identify",
            )),
        }?;

        Ok(res)
    }
}

impl MultPartFile<'_> {
    pub async fn save_to<P: AsRef<Path>>(self, path: P) -> std::io::Result<File> {
        let filename = self.filename();
        let filepath = path.as_ref().join(filename);

        log::info!("Saving file to {}", filepath.as_path().to_string_lossy());
        let mut file = rocket::tokio::fs::File::create(filepath).await?;

        AsyncWriteExt::write_all(&mut file, &self.data).await?;
        //let res = self.data.into_file(path.as_ref().join(filename)).await;
        Ok(file)
    }
    pub fn filename(&self) -> String {
        format!("{}.{}", self.filename, self.file_ext())
    }
    pub fn file_ext(&self) -> &str {
        &self.file_ext
    }
}
