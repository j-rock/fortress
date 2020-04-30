use crate::{
    dimensions::{
        LrDirection,
        Reverse,
    },
    items::types::{
        ItemTier1,
        ItemTier2,
        SkullType,
    },
    math::RandGen,
    render::SpriteSheetFrameId,
};

#[derive(Copy, Clone, PartialEq, Eq, Hash, Deserialize)]
pub enum ItemType {
    Skull(SkullType),
    Tier1(ItemTier1),
    Tier2(ItemTier2),
}

impl ItemType {
    pub fn render_reverse(self, facing_dir: LrDirection) -> Reverse {
        match self {
            Self::Skull(skull) => skull.render_reverse(facing_dir),
            Self::Tier1(tier1) => tier1.render_reverse(facing_dir),
            Self::Tier2(tier2) => tier2.render_reverse(facing_dir),
        }
    }

    pub fn sprite_frame_id(self) -> SpriteSheetFrameId {
        match self {
            Self::Skull(skull_type) => skull_type.sprite_frame_id(),
            Self::Tier1(tier1) => tier1.sprite_frame_id(),
            Self::Tier2(tier2) => tier2.sprite_frame_id(),
        }
    }

    pub fn random(rng: &mut RandGen) -> Self {
        if rng.flip_coin(0.75) {
            Self::Skull(SkullType::random(rng))
        } else if rng.flip_coin(0.75) {
            Self::Tier1(ItemTier1::random(rng))
        } else {
            Self::Tier2(ItemTier2::random(rng))
        }
    }
}