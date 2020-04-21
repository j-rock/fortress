use crate::render::{
    NamedSpriteSheet,
    SpriteSheetFrameId,
};

#[derive(Copy, Clone, PartialEq, Eq, Hash, Deserialize)]
pub enum SkullType {
    Mega,
    Regular,
}

impl SkullType {
    pub fn sprite_frame_id(self) -> SpriteSheetFrameId {
        match self {
            Self::Mega => SpriteSheetFrameId::new(String::from("item_mega_skull.png"), NamedSpriteSheet::SpriteSheet1),
            Self::Regular => SpriteSheetFrameId::new(String::from("item_skull.png"), NamedSpriteSheet::SpriteSheet1),
        }
    }
}
