#[derive(Clone, Deserialize)]
pub struct WraithConfig {
    pub size: (f32, f32),
    pub spawn_location: (f32, f32),
    pub density: f32,
    pub friction: f32,
    pub slashed_speed: f32,
}
