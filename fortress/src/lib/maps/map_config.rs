#[derive(Clone, Deserialize)]
pub struct MapConfig {
    pub cell_length: f64,
    pub light_center_height: f32,
    pub light_half_size: (f32, f32),
    pub light_color: (f32, f32, f32),
    pub light_attenuation: (f32, f32, f32),
    pub tile_scale: (f32, f32),
}
