use crate::{
    items::{
        ItemConfig,
        ItemPickup,
        state::ItemState,
    },
    render::{
        LightDependentSpriteRenderer,
        LightDependentSpriteData,
    },
};
use nalgebra::Vector2;

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

            sprite_renderer.queue(vec![LightDependentSpriteData {
                world_center_position,
                world_half_size,
                sprite_frame_id: state.item_pickup().sprite_frame_id(),
                frame: 0,
                unit_world_rotation: Vector2::new(0.0, 0.0),
                reverse: state.item_pickup().reverse(),
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

    pub fn item_pickup(&self, state: &ItemState) -> ItemPickup {
        state.item_pickup()
    }
}