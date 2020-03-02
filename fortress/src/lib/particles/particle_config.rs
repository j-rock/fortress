#[derive(Deserialize)]
pub struct BloodParticleConfig {
    pub particle_limit: usize,
    pub particles_per_event: usize,
    pub size: f32,
    pub gravity: f32,
    pub max_spread_speed: f32,
    pub start_height: f32,
    pub start_velocity_y: f32,
}

#[derive(Deserialize)]
pub struct ParticleConfig {
    pub initial_particle_events_limit_guess: usize,
    pub blood: BloodParticleConfig,
}