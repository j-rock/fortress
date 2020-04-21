use crate::{
    dimensions::{
        LrDirection,
        Reverse,
    },
    items::types::SkullType,
    render::SpriteSheetFrameId,
};

#[derive(Copy, Clone, PartialEq, Eq, Hash, Deserialize)]
pub enum ItemType {
    Skull(SkullType),
}

impl ItemType {
    pub fn render_reverse(self, facing_dir: LrDirection) -> Reverse {
        match self {
            Self::Skull(_) => {
                if facing_dir.is_left() {
                    Reverse::horizontally()
                } else {
                    Reverse::none()
                }
            }
        }
    }

    pub fn sprite_frame_id(self) -> SpriteSheetFrameId {
        match self {
            Self::Skull(skull_type) => skull_type.sprite_frame_id(),
        }
    }
}