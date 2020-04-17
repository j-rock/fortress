#[derive(Deserialize)]
pub struct ItemConfig {
    pub system_initial_capacity: usize,
    pub physical_radius: f64,
    pub physical_density: f64,
    pub render_scale: f32,
    pub bloom_intensity: f32,
}