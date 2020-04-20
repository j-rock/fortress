use crate::text::RasterSize;

#[derive(Deserialize)]
pub struct EnemySystemConfig {
    pub generator: EnemyGeneratorConfig,
    pub enemy: EnemyConfig,
    pub damage_text: DamageTextConfig,
}

#[derive(Deserialize)]
pub struct EnemyGeneratorConfig {
    pub slab_initial_capacity_guess: usize,
    pub starting_health: i64,
    pub cooldown_duration_micros: i64,
    pub spawn_offset_distance: f64,
    pub max_concurrent_spawns: usize,
    pub generate_distance: f64,
    pub physical_radius: f64,
    pub physical_density: f64,
    pub render_scale: f32,
    pub num_sprite_frames: usize,
    pub light_offset: (f32, f32, f32),
    pub light_color: (f32, f32, f32),
    pub light_attenuation: (f32, f32, f32),
    pub blood_color: (f32, f32, f32),
    pub num_blood_particles_per_hit: u32,
    pub death_screen_shake_intensity: f32,
}

#[derive(Deserialize)]
pub struct EnemyConfig {
    pub slab_initial_capacity_guess: usize,
    pub starting_health: i64,
    pub dying_duration_micros: i64,
    pub dying_frame_duration_micros: i64,
    pub walk_frame_duration_micros: i64,
    pub stop_and_hit_distance: f64,
    pub anger_distance: f64,
    pub move_speed: f64,
    pub physical_radius: f64,
    pub physical_density: f64,
    pub render_scale: f32,
    pub blood_color: (f32, f32, f32),
    pub num_blood_particles_per_hit: u32,
}

#[derive(Deserialize)]
pub struct DamageTextConfig {
    pub initial_capacity: usize,
    pub start_velocity: (f32, f32, f32),
    pub start_height: f32,
    pub vertical_acceleration: f32,
    pub text_expiry_duration_micros: i64,
    pub raster_size: RasterSize,
    pub color: (f32, f32, f32),
    pub shadow_color: (f32, f32, f32),
    pub shadow_offset: (f32, f32, f32),
}
