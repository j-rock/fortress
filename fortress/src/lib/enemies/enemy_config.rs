#[derive(Deserialize)]
pub struct EnemySystemConfig {
    pub generator: EnemyGeneratorConfig,
    pub enemy: EnemyConfig,
}

#[derive(Deserialize)]
pub struct EnemyGeneratorConfig {
    pub slab_initial_capacity_guess: usize,
    pub starting_health: i64,
    pub cooldown_duration_micros: i64,
    pub spawn_offset_distance: f64,
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
    pub light_duration_micros: i64,
    pub light_elevation: f32,
    pub light_color: (f32, f32, f32),
    pub light_attenuation: (f32, f32, f32),
    pub blood_color: (f32, f32, f32),
    pub num_blood_particles_per_hit: u32,
}
