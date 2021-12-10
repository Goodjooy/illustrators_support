
use sea_orm::Set;

use crate::{
    entity::illustrator_acts,
    utils::{multpart::MultPartFile, MaxLimitString},
};

#[derive(FromForm)]
pub struct ArtNew<'s> {
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
    pub file: MultPartFile<'s>,
}
impl<'s> ArtNew<'s> {
    pub fn into_save(self, iid: i64) -> (MultPartFile<'s>, ArtSaved) {
        let res = ArtSaved {
            iid,
            src: self.src.into().clone(),
            file: self.file.filename(),
        };
        (self.file, res)
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
            is_suit: Set(false as i8),
            src: Set(self.src),
            pic: Set(self.file),
            ..Default::default()
        }
    }
}
