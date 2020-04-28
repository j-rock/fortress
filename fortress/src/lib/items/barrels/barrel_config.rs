#[derive(Deserialize)]
pub struct BarrelConfig {
    pub physical_radius: f64,
    pub physical_density: f64,
    pub render_scale: (f32, f32),
    pub num_strikes_health: i64,
    pub blood_color: (f32, f32, f32),
    pub num_blood_particles_per_hit: u32,
}