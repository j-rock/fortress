#[derive(Clone, Deserialize)]
pub struct WraithConfig {
    pub size: (i32, i32),
    pub spawn_location: (i32, i32),
    pub density: f32,
    pub friction: f32,
    pub slashed_speed: f32,
}
