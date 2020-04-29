#[derive(Clone, Deserialize)]
pub struct MapConfig {
    pub cell_length: f64,
    pub bevel_height: f32,
    pub inner_beveled_hex_scale: f32,

    pub light_center_height: f32,
    pub light_half_size: (f32, f32),
    pub light_color: (f32, f32, f32),
    pub light_attenuation: (f32, f32, f32),
    pub light_bloom_intensity: f32,

    pub tile_scale: (f32, f32),
    pub stream_cell_min_elevation: f32,
    pub map_file: MapFileConfig,
}

#[derive(Clone, Deserialize)]
pub struct MapFileConfig {
    pub num_fragments: usize,
    pub terrain_count_guess: usize,
    pub spawn_count_guess: usize,
    pub lights_count_guess: usize,
    pub generators_count_guess: usize,
    pub barrel_count_guess: usize,
}
