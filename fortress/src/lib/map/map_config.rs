use map::Platform;

#[derive(Deserialize)]
pub struct MapConfig {
    pub platforms: Vec<Platform>,
}
