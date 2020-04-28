use crate::render::{
    NamedSpriteSheet,
    SpriteSheetFrameId,
};
use enum_iterator::IntoEnumIterator;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Deserialize, IntoEnumIterator)]
pub enum SkullType {
    Mega,
    Regular,
}

impl SkullType {
    pub fn all_values() -> impl Iterator<Item = Self> {
        Self::into_enum_iter()
    }

    pub fn sprite_frame_id(self) -> SpriteSheetFrameId {
        match self {
            Self::Mega => SpriteSheetFrameId::new(String::from("item_mega_skull.png"), NamedSpriteSheet::SpriteSheet1),
            Self::Regular => SpriteSheetFrameId::new(String::from("item_skull.png"), NamedSpriteSheet::SpriteSheet1),
        }
    }
}
