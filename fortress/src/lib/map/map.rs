use app::StatusOr;
use control::Controller;
use dimensions::time::DeltaTime;
use file::{
    ConfigWatcher,
    SimpleConfigManager,
};
use glm;
use liquidfun;
use map::{
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
    config_manager: SimpleConfigManager<MapConfig>,
    state: MapState,
}

impl Map {
    pub fn new(config_watcher: &mut ConfigWatcher, physics_sim: &mut PhysicsSimulation) -> StatusOr<Map> {
        let config_manager = SimpleConfigManager::from_config_resource(config_watcher, "map.conf")?;

        let state = {
            let config = config_manager.get();
            let map_body = MapBody::new(config, physics_sim.registrar(), physics_sim.get_world_mut());
            MapState::new(config.clone(), map_body)
        };

        Ok(Map {
            config_manager,
            state,
        })
    }

    pub fn register(&mut self) {
        let map: *const Map = self as *const Map;
        self.state.register(map);
    }

    pub fn pre_update(&mut self, _controller: &Controller, _dt: DeltaTime) {
        if self.config_manager.update() {
            self.redeploy();
        }
    }

    fn redeploy(&mut self) {
        {
            let config = self.config_manager.get();
            let mut world = self.state.body.platform_body.data_setter.get_world();
            let map_body = MapBody::new(config, self.state.body.platform_body.registrar.clone(), &mut world);
            self.state = MapState::new(config.clone(), map_body);
        }
        self.register();
    }

    pub fn draw(&mut self, box_renderer: &mut BoxRenderer) {
        let boxes: Vec<BoxData> = self.state.body.platform_body.data_setter.get_fixture_iterator().map(|fixture| -> BoxData {
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