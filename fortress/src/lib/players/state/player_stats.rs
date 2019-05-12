use crate::{
    dimensions::{
        Damage,
        time::{
            self,
            DeltaTime,
            Microseconds,
        },
    },
    items::{
        ItemPickup,
        ItemType,
    },
    players::PlayerConfig,
    render::{
        EasingFn,
        FullyIlluminatedSpriteData,
        FullyIlluminatedSpriteRenderer,
        PointLight,
    }
};
use generational_slab::Slab;
use glm;
use nalgebra::{
    Point2,
    Vector2,
};

pub struct PlayerStats {
    base_move_speed: f64,
    move_speed_level: usize,
    base_bullet_speed: f64,
    bullet_speed_level: usize,
    base_bullet_damage: Damage,
    bullet_damage_level: usize,
    base_bullet_knockback_strength: f64,
    bullet_knockback_strength_level: usize,
    base_normal_firing_period: Microseconds,
    normal_firing_period_level: usize,
    base_special_firing_period: Microseconds,
    special_firing_period_level: usize,
    skulls_collected: usize,

    collected_item_animations: Slab<CollectedItemAnimation>,
}

impl PlayerStats {
    pub fn new(config: &PlayerConfig) -> PlayerStats {
        PlayerStats {
            base_move_speed: config.base_move_speed,
            move_speed_level: 1,
            base_bullet_speed: config.bullet_speed,
            bullet_speed_level: 1,
            base_bullet_damage: Damage::new(config.bullet_damage),
            bullet_damage_level: 1,
            base_bullet_knockback_strength: config.bullet_knockback_strength,
            bullet_knockback_strength_level: 1,
            base_normal_firing_period: config.bullet_normal_firing_period_micros,
            normal_firing_period_level: 1,
            base_special_firing_period: config.bullet_special_firing_period_micros,
            special_firing_period_level: 1,
            skulls_collected: 0,

            collected_item_animations: Slab::with_capacity(config.item_collection_animation_num_concurrent_guess),
        }
    }

    pub fn pre_update(&mut self, config: &PlayerConfig, dt: DeltaTime) {
        let finished_animation_keys: Vec<_> = self.collected_item_animations
            .iter_mut()
            .filter_map(|(key, collected_item_animation)| {
                collected_item_animation.time_elapsed += dt.as_microseconds();
                if collected_item_animation.time_elapsed < config.item_collection_animation_duration_micros {
                    return None;
                }
                Some(key)
            })
            .collect();

        for key in finished_animation_keys.into_iter() {
            self.collected_item_animations.remove(key);
        }
    }

    pub fn populate_lights(&self, config: &PlayerConfig, player_center: Point2<f64>, lights: &mut Vec<PointLight>) {
        let mut queue_data: Vec<_> = self.collected_item_animations.iter()
            .map(|(_key, collected_item_animation)| {
                collected_item_animation.point_light(config, player_center)
            })
            .collect();
        lights.append(&mut queue_data);
    }

    pub fn queue_draw(&self, config: &PlayerConfig, player_center: Point2<f64>, full_light: &mut FullyIlluminatedSpriteRenderer) {
        let queue_data: Vec<_> = self.collected_item_animations.iter()
            .map(|(_key, collected_item_animation)| {
                collected_item_animation.sprite_data(config, player_center)
            })
            .collect();
        full_light.queue(queue_data);
    }

    pub fn get_move_speed(&self) -> f64 {
        self.base_move_speed * (self.move_speed_level as f64)
    }

    pub fn get_bullet_speed(&self) -> f64 {
        self.base_bullet_speed * (self.bullet_speed_level as f64)
    }

    pub fn get_normal_firing_period(&self) -> time::Microseconds {
        self.base_normal_firing_period - (self.normal_firing_period_level as time::Microseconds) * time::milliseconds(5)
    }

    pub fn get_special_firing_period(&self) -> time::Microseconds {
        self.base_special_firing_period - (self.special_firing_period_level as time::Microseconds) * time::milliseconds(5)
    }

    pub fn get_bullet_damage(&self) -> Damage {
        Damage::new(self.base_bullet_damage.value() * (self.bullet_damage_level as i64))
    }

    pub fn get_knockback_strength(&self) -> f64 {
        self.base_bullet_knockback_strength * (self.bullet_knockback_strength_level as f64)
    }

    pub fn collect_item(&mut self, item_pickup: ItemPickup) {
        self.collected_item_animations.insert(CollectedItemAnimation {
            item_pickup,
            time_elapsed: 0,
        });

        match item_pickup.item_type() {
            ItemType::MegaSkull => {
                self.skulls_collected += 5;
            },
            ItemType::Skull => {
                self.skulls_collected += 1;
            },
        }
    }
}

struct CollectedItemAnimation {
    item_pickup: ItemPickup,
    time_elapsed: Microseconds,
}

impl CollectedItemAnimation {
    pub fn point_light(&self, config: &PlayerConfig, player_center: Point2<f64>) -> PointLight {
        PointLight {
            position: self.world_center_position(config, player_center),
            color: self.item_pickup.light_color(),
            attenuation: glm::vec3(config.item_collection_attenuation.0, config.item_collection_attenuation.1, config.item_collection_attenuation.2),
        }
    }

    pub fn sprite_data(&self, config: &PlayerConfig, player_center: Point2<f64>) -> FullyIlluminatedSpriteData {
        FullyIlluminatedSpriteData {
            world_center_position: self.world_center_position(config, player_center),
            world_half_size: glm::vec2(config.item_collection_render_radius, config.item_collection_render_radius),
            sprite_frame_id: self.item_pickup.sprite_frame_id(),
            frame: 0,
            unit_world_rotation: Vector2::new(0.0, 0.0),
            reverse: self.item_pickup.reverse(),
        }
    }

    fn world_center_position(&self, config: &PlayerConfig, player_center: Point2<f64>) -> glm::Vec3 {
        let t = self.time_elapsed as f32 / (config.item_collection_animation_duration_micros as f32);
        let spin_radius = EasingFn::ease_out_quad(1.0 - t) * config.item_collection_animation_spin_radius;
        let spin_speed = 2.0 * std::f32::consts::PI * config.item_collection_animation_spin_max_speed;
        let x_pos = player_center.x as f32 + spin_radius * (spin_speed * t).cos();
        let y_pos = EasingFn::ease_out_quad(t) * config.item_collection_animation_max_height;
        let z_pos = player_center.y as f32 + spin_radius * (spin_speed * t).sin();

        glm::vec3(x_pos, y_pos, -z_pos)
    }
}
