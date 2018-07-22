use serde_json;

#[derive(Deserialize)]
struct Platform {
    top_left_x: i32,
    top_left_y: i32,
    width: i32,
    height: i32
}

#[derive(Deserialize)]
pub struct MapConfig {
    platforms: Vec<Platform>
}

pub struct Map {
}

impl Map {
    pub fn new(config: MapConfig) -> Map {
       Map {
       }
    }
}