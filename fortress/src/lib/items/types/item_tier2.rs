use crate::{
    dimensions::{
        LrDirection,
        Reverse,
    },
    render::{
        NamedSpriteSheet,
        SpriteSheetFrameId,
    },
};

#[derive(Copy, Clone, PartialEq, Eq, Hash, Deserialize)]
pub enum ItemTier2 {
    CritMultiplierBoost,
}

impl ItemTier2 {
    pub fn render_reverse(self, direction: LrDirection) -> Reverse {
        if direction.is_left() {
            Reverse::none()
        } else {
            Reverse::horizontally()
        }
    }

    pub fn sprite_frame_id(self) -> SpriteSheetFrameId {
        let image = match self {
            Self::CritMultiplierBoost => "crit_multiplier_potion.png",
        };
        SpriteSheetFrameId::new(String::from(image), NamedSpriteSheet::SpriteSheet1)
    }

    pub fn random() -> Self {
        Self::CritMultiplierBoost
    }
}
