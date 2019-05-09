use crate::{
    dimensions::{
        LrDirection,
        Reverse,
    },
    items::ItemType,
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
        match self.item_type {
            ItemType::Skull | ItemType::MegaSkull => {
                if self.facing_dir.is_left() {
                    Reverse::horizontally()
                } else {
                    Reverse::none()
                }
            },
        }
    }

    pub fn sprite_frame_id(&self) -> SpriteSheetFrameId {
        self.item_type.sprite_frame_id()
    }

    pub fn light_color(&self) -> glm::Vec3 {
        self.item_type.light_color()
    }
}