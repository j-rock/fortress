#[derive(Deserialize)]
pub struct CameraConfig {
    pub zoom: f32,
    pub z_near: f32,
    pub z_far: f32,
    pub lookat: (f32, f32, f32),
    pub right: (f32, f32, f32),
    pub camera_pos_offset: (f32, f32),
    pub initial_position_when_no_players: (f64, f64, f64),

    pub physical_no_move_half_lengths: (f64, f64),
    pub physical_follow_player_factor: f64,

    pub stream_inside_half_extents: (f64, f64),
    pub stream_margin_length: f64,
    pub stream_light_margin_length: f64,
}

