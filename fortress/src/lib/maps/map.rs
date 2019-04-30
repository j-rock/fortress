use crate::{
    app::StatusOr,
    enemies::EnemyGeneratorSpawn,
    file::{
        ConfigWatcher,
        SimpleConfigManager,
    },
    maps::{
        MapConfig,
        MapFile,
        MapState,
    },
    physics::PhysicsSimulation,
    render::{
        HexRenderer,
        PointLight,
        FullyIlluminatedSpriteRenderer,
    },
};
use nalgebra::Point2;

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
            MapState::new(config, map_file, physics_sim)
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

    pub fn populate_lights(&self, lights: &mut Vec<PointLight>) {
        let config = self.map_config_manager.get();
       self.map_state.populate_lights(config, lights);
    }

    pub fn queue_draw(&mut self, hex_renderer: &mut HexRenderer, sprite_renderer: &mut FullyIlluminatedSpriteRenderer) {
        let config = self.map_config_manager.get();
        self.map_state.queue_draw(config, hex_renderer, sprite_renderer);
    }

    pub fn spawns(&self) -> Vec<Point2<f64>> {
        self.map_state.spawns()
    }

    pub fn enemy_generator_spawns(&self) -> Vec<EnemyGeneratorSpawn> {
        self.map_state.enemy_generator_spawns()
    }

    fn redeploy(&mut self, physics_sim: &mut PhysicsSimulation) {
        {
            let config = self.map_config_manager.get();
            let map_file = self.map_file_manager.get();
            self.map_state = MapState::new(config, map_file, physics_sim);
        }
    }
}