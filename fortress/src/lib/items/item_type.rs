use crate::render::{
    NamedSpriteSheet,
    SpriteSheetFrameId,
};
use glm;

#[derive(Copy, Clone)]
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

    pub fn light_color(&self) -> glm::Vec3 {
        match self {
            ItemType::MegaSkull => glm::vec3(1.0, 1.0, 0.0),
            ItemType::Skull => glm::vec3(1.0, 1.0, 1.0),
        }
    }
}