use crate::{
    items::{
        ItemPickup,
        ItemConfig,
        state::ItemBody,
    },
    render::PointLight,
};
use glm;
use nalgebra::Point2;

pub struct ItemState {
    body: ItemBody,
    item_pickup: ItemPickup,
}

impl ItemState {
    pub fn new(body: ItemBody, item_pickup: ItemPickup) -> ItemState {
        ItemState {
            body,
            item_pickup,
        }
    }

    pub fn point_light(&self, config: &ItemConfig) -> Option<PointLight> {
        let position = self.position()?;
        Some(PointLight::new(
            glm::vec3(position.x as f32, config.light_elevation, -position.y as f32),
            self.item_pickup.light_color(config),
            glm::vec3(config.light_attenuation.0, config.light_attenuation.1, config.light_attenuation.2)))
    }

    pub fn item_pickup(&self) -> ItemPickup {
        self.item_pickup
    }

    pub fn position(&self) -> Option<Point2<f64>> {
        self.body.position()
    }
}