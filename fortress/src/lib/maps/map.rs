use app::StatusOr;
use file::{
    ConfigWatcher,
    SimpleConfigManager,
};
use glm;
use liquidfun;
use maps::{
    file::MapFile,
    MapConfig,
    MapState,
    state::MapBody,
};
use physics::PhysicsSimulation;
use render::{
    BoxData,
    BoxRenderer,
};

pub struct Map {
    map_config_manager: SimpleConfigManager<MapConfig>,
    map_file_manager:  SimpleConfigManager<MapFile>,
    state: MapState,
}

impl Map {
    pub fn new(config_watcher: &mut ConfigWatcher, physics_sim: &mut PhysicsSimulation) -> StatusOr<Map> {
        let map_config_manager = SimpleConfigManager::<MapConfig>::from_config_resource(config_watcher, "map.conf")?;
        let map_file_manager = {
            let config = map_config_manager.get();
            SimpleConfigManager::<MapFile>::from_resource_path(config_watcher, config.map_label.to_path())?
        };

        let state = {
            let config = map_config_manager.get();
            let map_file = map_file_manager.get();
            let map_body = MapBody::new(config, map_file, physics_sim);
            MapState::new(map_body)
        };

        Ok(Map {
            map_config_manager,
            map_file_manager,
            state,
        })
    }

    pub fn register(&mut self) {
        let map: *const Map = self as *const Map;
        self.state.register(map);
    }

    pub fn pre_update(&mut self, physics_sim: &mut PhysicsSimulation) -> bool {
        if self.map_config_manager.update() || self.map_file_manager.update() {
            self.redeploy(physics_sim);
            true
        } else {
            false
        }
    }

    pub fn get_player_spawns(&self) -> Vec<liquidfun::box2d::common::math::Vec2> {
        let cell_len = self.map_config_manager.get().map_file_cell_length;
        self.map_file_manager
            .get()
            .spawns
            .iter()
            .map(|grid_loc| {
                grid_loc.to_2d(cell_len)
            })
            .collect()
    }

    pub fn get_buff_box_spawns(&self) -> Vec<liquidfun::box2d::common::math::Vec2> {
        let cell_len = self.map_config_manager.get().map_file_cell_length;
        self.map_file_manager
            .get()
            .buff_boxes
            .iter()
            .map(|grid_loc| {
                grid_loc.to_2d(cell_len)
            })
            .collect()
    }

    fn redeploy(&mut self, physics_sim: &mut PhysicsSimulation) {
        {
            let config = self.map_config_manager.get();
            let map_file = self.map_file_manager.get();
            let map_body = MapBody::new(config, map_file, physics_sim);
            self.state = MapState::new(map_body);
        }
        self.register();
    }

    pub fn draw(&mut self, box_renderer: &mut BoxRenderer) {
        let boxes: Vec<BoxData> = self.state.body.wall_body.data_setter.get_fixture_iterator().map(|fixture| -> BoxData {
            let mut polygon =  liquidfun::box2d::collision::shapes::polygon_shape::from_shape(fixture.get_shape());
            let vertices: Vec<liquidfun::box2d::common::math::Vec2> = polygon.get_vertex_iterator().collect();
            let (min_vertex, max_vertex) = (vertices[0], vertices[2]);

            let half_size = glm::vec2((max_vertex.x - min_vertex.x) / 2.0, (max_vertex.y - min_vertex.y) / 2.0);
            let position = glm::vec2(half_size.x + min_vertex.x, half_size.y + min_vertex.y);

            BoxData {
                position,
                half_size,
                rgba_tl: glm::vec4(0.3, 0.0, 0.4, 0.0),
                rgba_tr: glm::vec4(0.0, 0.8, 0.0, 0.0),
                rgba_bl: glm::vec4(0.5, 0.5, 1.0, 0.0),
                rgba_br: glm::vec4(1.0, 0.5, 0.0, 0.0),
            }}).collect();

        box_renderer.queue(boxes.as_slice());
    }
}