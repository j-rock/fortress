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
    render::hex_renderer::HexRenderer,
};

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

    fn redeploy(&mut self, physics_sim: &mut PhysicsSimulation) {
        {
            let config = self.map_config_manager.get();
            let map_file = self.map_file_manager.get();
            self.map_state = MapState::new(config, map_file, physics_sim);
        }
    }

    pub fn draw(&mut self, projection_view: &glm::Mat4) {
        self.map_state.draw(&mut self.renderer);

        self.renderer.draw_begin();
        self.renderer.draw(projection_view);
        self.renderer.draw_end();
    }
}