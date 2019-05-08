use crate::render::NamedSpriteSheet;

#[derive(Copy, Clone)]
pub enum ItemType {
    MegaSkull,
    Skull,
}

impl ItemType {
    pub fn sprite_info(self) -> (String, NamedSpriteSheet) {
        match self {
            ItemType::MegaSkull => (String::from("item_mega_skull.png"), NamedSpriteSheet::SpriteSheet1),
            ItemType::Skull => (String::from("item_skull.png"), NamedSpriteSheet::SpriteSheet1),
        }
    }
}