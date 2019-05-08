use crate::{
    dimensions::Reverse,
    items::{
        ItemConfig,
        ItemType,
        state::ItemState,
    },
    render::{
        LightDependentSpriteRenderer,
        LightDependentSpriteData,
        SpriteSheetFrameId,
    },
};

pub enum ItemStateMachine {
    AwaitingCollection,
    Collected
}

impl Default for ItemStateMachine {
    fn default() -> ItemStateMachine {
        ItemStateMachine::AwaitingCollection
    }
}

impl ItemStateMachine {
    pub fn pre_update(&mut self, _state: &ItemState) -> Option<ItemStateMachine> {
        None
    }

    pub fn post_update(&mut self, _state: &ItemState) -> Option<ItemStateMachine> {
        None
    }

    pub fn queue_draw(&self, config: &ItemConfig, state: &ItemState, sprite_renderer: &mut LightDependentSpriteRenderer) {
        if let Some(position) = state.position() {
            let world_half_size = glm::vec2(config.item_physical_radius as f32, config.item_physical_radius as f32) * config.item_render_scale;
            let world_center_position = glm::vec3(position.x as f32, world_half_size.y, -position.y as f32);

            let reverse = if state.facing_dir().is_left() {
                Reverse::horizontally()
            } else {
                Reverse::none()
            };

            let (name, sprite_sheet) = state.item_type().sprite_info();

            sprite_renderer.queue(vec![LightDependentSpriteData {
                world_center_position,
                world_half_size,
                sprite_frame_id: SpriteSheetFrameId {
                    name,
                    sprite_sheet,
                },
                frame: 0,
                rotation: 0.0,
                reverse,
            }]);
        }
    }

    pub fn collect(&mut self) {
        *self = ItemStateMachine::Collected;
    }

    pub fn collected(&self) -> bool {
        match self {
            ItemStateMachine::Collected => true,
            _ => false,
        }
    }

    pub fn item_type(&self, state: &ItemState) -> ItemType {
        state.item_type()
    }
}