use crate::{
    app::StatusOr,
    file::{
        ConfigWatcher,
        SimpleConfigManager,
    },
    maps::{
        MapConfig,
        MapFile,
        MapState,
        state::MapBody,
    },
    physics::PhysicsSimulation,
};

pub struct Map {
    map_config_manager: SimpleConfigManager<MapConfig>,
    map_file_manager:  SimpleConfigManager<MapFile>,
    map_state: MapState,
}

impl Map {
    pub fn new(config_watcher: &mut ConfigWatcher, physics_sim: &mut PhysicsSimulation) -> StatusOr<Map> {
        let map_config_manager = SimpleConfigManager::<MapConfig>::from_config_resource(config_watcher, "map.conf")?;
        let map_file_manager = {
            let config = map_config_manager.get();
            SimpleConfigManager::<MapFile>::from_resource_path(config_watcher, config.map_label.to_path())?
        };

        let map_state = {
            let config = map_config_manager.get();
            let map_file = map_file_manager.get();
            let map_body = MapBody::new(config, map_file, physics_sim);
            MapState::new(map_body)
        };

        Ok(Map {
            map_config_manager,
            map_file_manager,
            map_state,
        })
    }

    pub fn pre_update(&mut self, physics_sim: &mut PhysicsSimulation) -> bool {
        if self.map_config_manager.update() || self.map_file_manager.update() {
            self.redeploy(physics_sim);
            true
        } else {
            false
        }
    }

    fn redeploy(&mut self, physics_sim: &mut PhysicsSimulation) {
        {
            let config = self.map_config_manager.get();
            let map_file = map_file_manager.get();
            let map_body = MapBody::new(config, map_file, physics_sim);
            self.map_state = MapState::new(map_body)
        }
    }

    pub fn draw(&mut self) {
    }
}