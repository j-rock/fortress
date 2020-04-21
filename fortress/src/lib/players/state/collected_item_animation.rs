use crate::{
    dimensions::time::{
        DeltaTime,
        Timer,
    },
    items::{
        ItemConfig,
        ItemPickup,
    },
    math::EasingFn,
    players::PlayerItemConfig,
    render::{
        FullyIlluminatedSpriteData,
        FullyIlluminatedSpriteRenderer,
        PointLight,
        PointLights,
    }
};
use glm;
use nalgebra::{
    Point2,
    Vector2,
};

pub struct CollectedItemAnimation {
    item_pickup: Vec<ItemPickup>,
    timer: Vec<Timer>,
}

impl CollectedItemAnimation {
    pub fn new(config: &PlayerItemConfig) -> Self {
        CollectedItemAnimation {
            item_pickup: Vec::with_capacity(config.collect_animation_num_concurrent_guess),
            timer: Vec::with_capacity(config.collect_animation_num_concurrent_guess),
        }
    }

    pub fn pre_update(&mut self, dt: DeltaTime) {
        (0..self.item_pickup.len())
            .rev()
            .for_each(|idx| {
                let ref mut timer = self.timer[idx];
                timer.tick(dt);
                if timer.is_expired() {
                    self.item_pickup.swap_remove(idx);
                    self.timer.swap_remove(idx);
                    return;
                }
            });
    }

    pub fn add_animation(&mut self, config: &PlayerItemConfig, item_pickup: ItemPickup) {
        self.item_pickup.push(item_pickup);
        self.timer.push(Timer::new(config.collect_animation_duration_micros));
    }

    pub fn populate_lights(&self,
                           config: &PlayerItemConfig,
                           item_config: &ItemConfig,
                           player_center: Point2<f64>,
                           lights: &mut PointLights) {
        let queue_data =
            (0..self.item_pickup.len())
                .map(|idx| {
                    let t = self.timer[idx].as_completion_fraction_of(config.collect_animation_duration_micros);
                    let light_strength = EasingFn::ease_out_quintic(t);
                    let position = Self::world_center_position(config, player_center, t);
                    let color = self.item_pickup[idx].light_color(item_config) * light_strength;
                    let attenuation = glm::vec3(config.collect_attenuation.0, config.collect_attenuation.1, config.collect_attenuation.2);
                    PointLight::new(position, color, attenuation)
                });

        lights.append(queue_data);
    }

    pub fn queue_draw(&self, config: &PlayerItemConfig, player_center: Point2<f64>, full_light: &mut FullyIlluminatedSpriteRenderer) {
        let queue_data =
            (0..self.item_pickup.len())
                .map(|idx| {
                    let t = self.timer[idx].as_completion_fraction_of(config.collect_animation_duration_micros);
                    let item_pickup = self.item_pickup[idx];
                    FullyIlluminatedSpriteData {
                        world_center_position: Self::world_center_position(config, player_center, t),
                        world_half_size: glm::vec2(config.collect_render_radius, config.collect_render_radius),
                        sprite_frame_id: item_pickup.sprite_frame_id(),
                        frame: 0,
                        unit_world_rotation: Vector2::new(0.0, 0.0),
                        reverse: item_pickup.reverse(),
                        bloom_intensity: config.collect_animation_bloom_intensity,
                    }
                });

        full_light.queue(queue_data);
    }

    fn world_center_position(config: &PlayerItemConfig, player_center: Point2<f64>, t: f32) -> glm::Vec3 {
        let spin_radius = EasingFn::ease_out_quad(1.0 - t) * config.collect_animation_spin_radius;
        let spin_speed = 2.0 * std::f32::consts::PI * config.collect_animation_spin_max_speed;
        let x_pos = player_center.x as f32 + spin_radius * (spin_speed * t).cos();
        let y_pos = EasingFn::ease_out_quad(t) * config.collect_animation_max_height;
        let z_pos = player_center.y as f32 + spin_radius * (spin_speed * t).sin();
        glm::vec3(x_pos, y_pos, -z_pos)
    }
}
