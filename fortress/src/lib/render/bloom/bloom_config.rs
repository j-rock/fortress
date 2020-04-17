#[derive(Deserialize)]
pub struct BloomConfig {
    pub num_passes: usize,
    pub bloom_intensity_multiplier: f32,
}