use crate::{
    items::{
        ItemConfig,
        ItemPickup,
        state::ItemState,
    },
    render::{
        FullyIlluminatedSpriteData,
        FullyIlluminatedSpriteRenderer,
        PointLight,
    },
};
use nalgebra::Vector2;

pub enum ItemStateMachine {
    AwaitingCollection,
    Collected
}

impl Default for ItemStateMachine {
    fn default() -> Self {
        Self::AwaitingCollection
    }
}

impl ItemStateMachine {
    pub fn point_light(&self, config: &ItemConfig, state: &ItemState) -> Option<PointLight> {
        match self {
            Self::AwaitingCollection => state.point_light(config),
            Self::Collected => None,
        }
    }

    pub fn queue_draw(&self, config: &ItemConfig, state: &ItemState, sprite_renderer: &mut FullyIlluminatedSpriteRenderer) {
        if let Some(position) = state.position() {
            let world_half_size = glm::vec2(config.physical_radius as f32, config.physical_radius as f32) * config.render_scale;
            let world_center_position = glm::vec3(position.x as f32, world_half_size.y, -position.y as f32);

            sprite_renderer.queue(Some(FullyIlluminatedSpriteData {
                world_center_position,
                world_half_size,
                sprite_frame_id: state.item_pickup().sprite_frame_id(),
                frame: 0,
                unit_world_rotation: Vector2::new(0.0, 0.0),
                reverse: state.item_pickup().reverse(),
                bloom_intensity: config.bloom_intensity,
            }));
        }
    }

    pub fn collect(&mut self) {
        *self = Self::Collected;
    }

    pub fn collected(&self) -> bool {
        match self {
            Self::Collected => true,
            _ => false,
        }
    }

    pub fn item_pickup(&self, state: &ItemState) -> ItemPickup {
        state.item_pickup()
    }
}