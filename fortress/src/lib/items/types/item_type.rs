use crate::{
    dimensions::{
        LrDirection,
        Reverse,
    },
    items::types::SkullType,
    math::RandGen,
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

    pub fn random(rng: &mut RandGen) -> Self {
        let skull_type = {
            let all_values: Vec<SkullType> = SkullType::all_values().collect();
            rng.choose_uniformly(all_values.as_slice()).clone()
        };
        Self::Skull(skull_type)
    }
}