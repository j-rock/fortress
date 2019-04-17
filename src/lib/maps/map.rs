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
    },
    physics::PhysicsSimulation,
    render::{
        hex_renderer::HexRenderer,
        PointLight,
        SpriteRenderer,
    },
};
use nalgebra::Point2;

pub struct Map {
    map_config_manager: SimpleConfigManager<MapConfig>,
    map_file_manager:  SimpleConfigManager<MapFile>,
    map_state: MapState,
    renderer: HexRenderer,
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

        let renderer = HexRenderer::new()?;

        Ok(Map {
            map_config_manager,
            map_file_manager,
            map_state,
            renderer,
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

    pub fn draw(&mut self, projection_view: &glm::Mat4, sprite_renderer: &mut SpriteRenderer, lights: &mut Vec<PointLight>) {
        let config = self.map_config_manager.get();
        self.map_state.queue_draw(config, &mut self.renderer, sprite_renderer, lights);
        self.renderer.draw(projection_view);
    }

    pub fn spawns(&self) -> Vec<Point2<f64>> {
        self.map_state.spawns()
    }

    fn redeploy(&mut self, physics_sim: &mut PhysicsSimulation) {
        {
            let config = self.map_config_manager.get();
            let map_file = self.map_file_manager.get();
            self.map_state = MapState::new(config, map_file, physics_sim);
        }
    }
}