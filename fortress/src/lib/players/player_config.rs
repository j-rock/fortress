use crate::players::Hero;
use std::collections::HashMap;

#[derive(Clone, Deserialize)]
pub struct PlayerConfig {
    pub physical_radius: f64,
    pub physical_density: f64,
    pub weapon_physical_offset: f64,
    pub fire_special_move_freeze_duration_micros: i64,
    pub switch_hero_duration_micros: i64,
    pub switch_hero_screen_shake_intensity: f32,
}

#[derive(Clone, Deserialize)]
pub struct PlayerHeroConfig {
    pub base_move_speed: f64,
    pub fire_special_knockback_strength: f64,
    pub render_offset: (f32, f32),
    pub render_scale: (f32, f32),
    pub idle_image_name: String,
    pub idle_frame_duration_micros: i64,
    pub walking_image_name: String,
    pub walking_frame_duration_micros: i64,
    pub render_extra: Option<PlayerHeroExtraRenderConfig>,
}

#[derive(Clone, Deserialize)]
pub struct PlayerHeroExtraRenderConfig {
    pub bloom_intensity: f32,
    pub idle_image_extra_name: String,
    pub walking_image_extra_name: String,
}

#[derive(Clone, Deserialize)]
pub struct PlayerBulletConfig {
    pub normal_firing_period_micros: i64,
    pub special_firing_period_micros: i64,
    pub special_spread_radians: f64,
    pub special_num_shots: usize,
    pub special_screen_shake_intensity: f32,
    pub lifetime_duration_micros: i64,
    pub physical_radius: f64,
    pub speed: f64,
    pub damage: i64,
    pub knockback_strength: f64,
    pub render_width: f32,
    pub render_height: f32,
    pub render_elevation: f32,
    pub sprite_frame_duration_micros: i64,
    pub sprite_num_frames: usize,
    pub light_attenuation: (f32, f32, f32),
    pub bloom_intensity: f32,
    pub light_color_fire: (f32, f32, f32),
    pub light_color_poison: (f32, f32, f32),
    pub light_color_ice: (f32, f32, f32),
}

#[derive(Clone, Deserialize)]
pub struct PlayerItemConfig {
    pub collect_animation_num_concurrent_guess: usize,
    pub collect_animation_duration_micros: i64,
    pub collect_render_radius: f32,
    pub collect_animation_spin_radius: f32,
    pub collect_animation_spin_max_speed: f32,
    pub collect_animation_max_height: f32,
    pub collect_attenuation: (f32, f32, f32),
    pub collect_animation_bloom_intensity: f32,
}

#[derive(Clone, Deserialize)]
pub struct PlayerSystemConfig {
    pub player: PlayerConfig,
    pub hero: HashMap<Hero, PlayerHeroConfig>,
    pub bullet: PlayerBulletConfig,
    pub item: PlayerItemConfig,
}

