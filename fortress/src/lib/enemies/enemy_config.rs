#[derive(Deserialize)]
pub struct EnemyConfig {
    // Generators
    pub generators_slab_initial_capacity_guess: usize,

    pub generator_starting_health: i64,
    pub generator_cooldown_duration_micros: i64,
    pub generator_physical_radius: f64,
    pub generator_physical_density: f64,
    pub generator_offset_distance: f64,

    // Enemies
    pub enemies_slab_initial_capacity_guess: usize,

    pub enemy_starting_health: i64,
    pub enemy_dying_duration_micros: i64,

    pub enemy_physical_radius: f64,
    pub enemy_physical_density: f64,
}
