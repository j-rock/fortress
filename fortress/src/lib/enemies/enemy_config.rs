#[derive(Deserialize)]
pub struct EnemyConfig {
    // Generators
    pub generators_slab_initial_capacity_guess: usize,

    pub generator_starting_health: i64,
    pub generator_cooldown_duration_micros: i64,
    pub generator_offset_distance: f64,
    pub generator_physical_radius: f64,
    pub generator_physical_density: f64,
    pub generator_render_scale: f32,
    pub generator_num_sprite_frames: usize,

    // Enemies
    pub enemies_slab_initial_capacity_guess: usize,
    pub enemy_starting_health: i64,
    pub enemy_dying_duration_micros: i64,
    pub enemy_dying_frame_duration_micros: i64,
    pub enemy_walk_frame_duration_micros: i64,
    pub enemy_stop_and_hit_distance: f64,
    pub enemy_anger_distance: f64,
    pub enemy_move_speed: f64,
    pub enemy_physical_radius: f64,
    pub enemy_physical_density: f64,
    pub enemy_render_scale: f32,
}
