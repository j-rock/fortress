use crate::{
    dimensions::{
        LrDirection,
        Reverse,
    },
    items::{
        ItemConfig,
        ItemType,
    },
    math::RandGen,
    render::SpriteSheetFrameId,
};
use glm;

#[derive(Copy, Clone)]
pub struct ItemPickup {
    item_type: ItemType,
    facing_dir: LrDirection,
}

impl ItemPickup {
    pub fn new(item_type: ItemType, facing_dir: LrDirection) -> ItemPickup {
       ItemPickup {
           item_type,
           facing_dir,
       }
    }

    pub fn item_type(&self) -> ItemType {
        self.item_type
    }

    pub fn reverse(&self) -> Reverse {
        self.item_type.render_reverse(self.facing_dir)
    }

    pub fn sprite_frame_id(&self) -> SpriteSheetFrameId {
        self.item_type.sprite_frame_id()
    }

    pub fn light_color(&self, config: &ItemConfig) -> glm::Vec3 {
        let color = config.item_type_light_color
            .get(&self.item_type)
            .unwrap_or(&(1.0, 1.0, 1.0));
        glm::vec3(color.0, color.1, color.2)
    }

    pub fn random(rng: &mut RandGen) -> Self {
        let direction = LrDirection::random(rng);
        Self::new(ItemType::random(rng), direction)
    }
}