use crate::render::{
    NamedSpriteSheet,
    SpriteSheetFrameId,
};

#[derive(Copy, Clone, PartialEq, Eq, Hash, Deserialize)]
pub enum ItemType {
    MegaSkull,
    Skull,
}

impl ItemType {
    pub fn sprite_frame_id(self) -> SpriteSheetFrameId {
        match self {
            ItemType::MegaSkull => SpriteSheetFrameId::new(String::from("item_mega_skull.png"), NamedSpriteSheet::SpriteSheet1),
            ItemType::Skull => SpriteSheetFrameId::new(String::from("item_skull.png"), NamedSpriteSheet::SpriteSheet1),
        }
    }
}