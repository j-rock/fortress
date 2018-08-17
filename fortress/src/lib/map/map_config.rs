#[derive(Clone, Deserialize)]
pub struct Platform {
    pub top_left_x: i32,
    pub top_left_y: i32,
    pub width: i32,
    pub height: i32
}

#[derive(Clone, Deserialize)]
pub struct MapConfig {
    pub friction: f32,
    pub platforms: Vec<Platform>,
}
