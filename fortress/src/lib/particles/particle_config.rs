#[derive(Deserialize)]
pub struct BloodParticleConfig {
    pub particle_limit: usize,
    pub size_range: (f32, f32),
    pub gravity: f32,
    pub max_spread_speed: f32,
    pub start_height: f32,
    pub start_velocity_y: f32,
    pub start_position_radius: f32,
    pub bloom_intensity: f32,
}

#[derive(Deserialize)]
pub struct SnowParticleConfig {
    pub particle_limit: usize,
    pub particle_generation_period_micros: i64,
    pub wind_direction_raw: (f32, f32, f32),
    pub wind_direction_max_angle_offset: f32,
    pub speed_range: (f32, f32),
    pub size_range: (f32, f32),
    pub color: (f32, f32, f32),
    pub start_position_offset: (f32, f32, f32),
    pub height_above_which_alpha_is_full: f32,
    pub bloom_intensity: f32,
}

#[derive(Deserialize)]
pub struct HeroSwitchParticleConfig {
    pub particle_limit: usize,
    pub particles_per_event: usize,
    pub size: f32,
    pub color: (f32, f32, f32),
    pub max_age_seconds: f64,
    pub starting_radial_offset: f64,
    pub starting_height_band: (f64, f64),
    pub xz_speed_band: (f64, f64),
    pub wave_speed_band: (f64, f64),
    pub wave_amplitude: f64,
    pub wave_phase_shift: f64,
    pub bloom_intensity: f32,
}

#[derive(Deserialize)]
pub struct ParticleConfig {
    pub initial_particle_events_limit_guess: usize,
    pub blood: BloodParticleConfig,
    pub snow: SnowParticleConfig,
    pub hero_switch: HeroSwitchParticleConfig,
}