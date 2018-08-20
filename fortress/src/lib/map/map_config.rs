#[derive(Clone, Deserialize)]
pub struct Platform {
    pub top_left_x: f32,
    pub top_left_y: f32,
    pub width: f32,
    pub height: f32
}

#[derive(Clone, Deserialize)]
pub struct MapConfig {
    pub friction: f32,
    pub platforms: Vec<Platform>,
}
