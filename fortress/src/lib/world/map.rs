use app::StatusOr;
use file::{
    ConfigLoader,
    ConfigWatcher,
    self,
};
use std;

#[derive(Deserialize)]
struct Platform {
    top_left_x: i32,
    top_left_y: i32,
    width: i32,
    height: i32
}

#[derive(Deserialize)]
struct MapData {
    platforms: Vec<Platform>
}

pub struct Map {
    config_loader: ConfigLoader<MapData>,
    map_data: MapData
}

impl Map {
    pub fn new(config_watcher: &mut ConfigWatcher) -> StatusOr<Map> {
        let map_config = file::util::resource_path("config", "map.conf");
        let mut config_loader = config_watcher.watch(map_config)?;
        let map_data = config_loader.force_load()?;
        Ok(Map {
            config_loader,
            map_data
        })
    }

    pub fn update(&mut self) {
        let reload_map = self.config_loader.try_load();
        match reload_map {
            Err(message) => println!("Error reloading map.conf: {}", message),
            Ok(None) => {},
            Ok(Some(map_data)) => {
                println!("Replacing map!");
                std::mem::replace(&mut self.map_data, map_data);
            }
        }
    }
}