use crate::{
    dimensions::{
        LrDirection,
        Reverse,
    },
    math::RandGen,
    render::{
        NamedSpriteSheet,
        SpriteSheetFrameId,
    },
};
use enum_iterator::IntoEnumIterator;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Deserialize, IntoEnumIterator)]
pub enum SkullType {
    Mega,
    Regular,
}

impl SkullType {
    pub fn render_reverse(self, facing_dir: LrDirection) -> Reverse {
        if facing_dir.is_left() {
            Reverse::horizontally()
        } else {
            Reverse::none()
        }
    }

    pub fn sprite_frame_id(self) -> SpriteSheetFrameId {
        match self {
            Self::Mega => SpriteSheetFrameId::new(String::from("item_mega_skull.png"), NamedSpriteSheet::SpriteSheet1),
            Self::Regular => SpriteSheetFrameId::new(String::from("item_skull.png"), NamedSpriteSheet::SpriteSheet1),
        }
    }

    pub fn random(rng: &mut RandGen) -> Self {
        match rng.ranged_i64(0, 2) {
            0 => Self::Mega,
            _ => Self::Regular,
        }
    }
}
