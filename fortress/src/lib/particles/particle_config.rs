#[derive(Deserialize)]
pub struct ParticleConfig {
    pub particle_capacity: usize,
    pub initial_particle_events_guess: usize,
    pub particle_start_height: f32,
    pub particle_max_spread_velocity: f32,
    pub particle_start_velocity_y: f32,
    pub particles_per_event: usize,
    pub particle_gravity: f32,
    pub particle_size: f32,
}