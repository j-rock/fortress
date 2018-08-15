use map::Platform;

#[derive(Deserialize)]
pub struct MapConfig {
    pub friction: f32,
    pub platforms: Vec<Platform>,
}
