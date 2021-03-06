use crate::{
    app::StatusOr,
    file::{
        ConfigWatcher,
        SimpleConfigManager,
    },
    maps::{
        MapConfig,
        MapFileManager,
        MapState,
        render::HexRenderer,
    },
    physics::PhysicsSimulation,
    render::{
        CameraGeometry,
        CameraStreamInfo,
        FullyIlluminatedSpriteRenderer,
        PointLights,
        SpriteSheetTextureManager,
    },
};
use nalgebra::Point2;

pub struct MapSystem {
    map_config_manager: SimpleConfigManager<MapConfig>,
    map_file_manager:  MapFileManager,
    map_state: MapState,
    hex_renderer: HexRenderer,
}

impl MapSystem {
    pub fn new(config_watcher: &mut ConfigWatcher, physics_sim: &mut PhysicsSimulation) -> StatusOr<MapSystem> {
        let map_config_manager = SimpleConfigManager::<MapConfig>::from_config_resource(config_watcher, "map.conf")?;
        let map_file_manager = {
            let config = map_config_manager.get();
            MapFileManager::new(&config.map_file, config_watcher)?
        };

        let (map_state, hex_renderer) = {
            let config = map_config_manager.get();
            let map_file = map_file_manager.get();
            let map_state = MapState::new(config, map_file, physics_sim);
            let hex_renderer = HexRenderer::new(config)?;
            (map_state, hex_renderer)
        };

        Ok(MapSystem {
            map_config_manager,
            map_file_manager,
            map_state,
            hex_renderer,
        })
    }

    pub fn pre_update(&mut self, physics_sim: &mut PhysicsSimulation) -> bool {
        if self.map_config_manager.update() || self.map_file_manager.update(&self.map_config_manager.get().map_file) {
            self.redeploy(physics_sim).is_ok()
        } else {
            false
        }
    }

    pub fn populate_lights(&self, lights: &mut PointLights) {
        let config = self.map_config_manager.get();
        self.map_state.populate_lights(config, lights);
    }

    pub fn queue_draw(&mut self, camera_stream_info: &CameraStreamInfo, sprite_renderer: &mut FullyIlluminatedSpriteRenderer) {
        let config = self.map_config_manager.get();
        self.map_state.queue_draw(config, camera_stream_info, &mut self.hex_renderer, sprite_renderer);
    }

    pub fn draw_terrain(&mut self, textures: &SpriteSheetTextureManager, lights: &PointLights, geometry: &CameraGeometry) {
        let config = self.map_config_manager.get();
        self.hex_renderer.draw(config, textures, lights, geometry);
    }

    pub fn spawns(&self) -> &[Point2<f64>] {
        self.map_state.player_spawns()
    }

    pub fn enemy_generators(&self) -> &[Point2<f64>] {
        self.map_state.enemy_generators()
    }

    pub fn barrels(&self) -> &[Point2<f64>] {
        self.map_state.barrels()
    }

    pub fn hex_cell_length(&self) -> f64 {
        self.map_config_manager.get().cell_length
    }

    fn redeploy(&mut self, physics_sim: &mut PhysicsSimulation) -> StatusOr<()> {
        let config = self.map_config_manager.get();
        let map_file = self.map_file_manager.get();
        self.map_state = MapState::new(config, map_file, physics_sim);
        self.hex_renderer = HexRenderer::new(config)?;
        Ok(())
    }
}