#[derive(Clone, Deserialize)]
pub struct WraithConfig {
    pub size: (f32, f32),
    pub spawn_location: (f32, f32),
    pub density: f32,
    pub friction: f32,
    pub starting_health: i64,
}
