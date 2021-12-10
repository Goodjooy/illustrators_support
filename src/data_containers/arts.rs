use sea_orm::Set;

use crate::{entity::illustrator_acts, utils::MaxLimitString};

#[derive(FromForm)]
pub struct ArtNew {
    #[field(name = uncased("source"))]
    #[field(name = uncased("pixiv-src"))]
    #[field(name = uncased("pixiv_src"))]
    #[field(name = uncased("img_src"))]
    #[field(name = uncased("src"))]
    pub src: MaxLimitString<256>,
    #[field(name = uncased("img"))]
    #[field(name = uncased("image"))]
    #[field(name = uncased("img-src"))]
    #[field(name = uncased("file"))]
    pub file: MaxLimitString<256>,
}
impl ArtNew {
    pub fn into_save(self, iid: i64) -> ArtSaved {
        let res = ArtSaved {
            iid,
            src: self.src.into(),
            file: self.file.into(),
        };
        res
    }
}

pub struct ArtSaved {
    iid: i64,
    src: String,
    file: String,
}

impl Into<illustrator_acts::ActiveModel> for ArtSaved {
    fn into(self) -> illustrator_acts::ActiveModel {
        illustrator_acts::ActiveModel {
            iid: Set(self.iid),
            src: Set(self.src),
            pic: Set(self.file),
            ..Default::default()
        }
    }
}
