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

#[derive(Copy, Clone, PartialEq, Eq, Hash, Deserialize)]
pub enum ItemTier2 {
    CritMultiplierBoost,
    SpecialFiringPeriodBoost,
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
            Self::SpecialFiringPeriodBoost => "special_firing_speed_glove.png",
        };
        SpriteSheetFrameId::new(String::from(image), NamedSpriteSheet::SpriteSheet1)
    }

    pub fn random(rng: &mut RandGen) -> Self {
        match rng.ranged_i64(0, 2) {
            0 => Self::CritMultiplierBoost,
            _ => Self::SpecialFiringPeriodBoost,
        }
    }
}
