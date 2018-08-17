use app::StatusOr;
use control::Controller;
use dimensions::time::DeltaTime;
use file::{
    ConfigWatcher,
    SimpleConfigManager,
    self,
};
use gl::{
    self,
    types::*,
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
    attribute,
    AttributeProgram,
    Attribute,
    ShaderProgram,
};

#[repr(C)]
struct PlatformAttr {
    bottom_left: glm::Vec2,
    top_right: glm::Vec2,
}

impl attribute::KnownComponent for PlatformAttr {
    fn component() -> (attribute::NumComponents, attribute::ComponentType) {
        (attribute::NumComponents::S4, attribute::ComponentType::FLOAT)
    }
}

pub struct Map {
    config_manager: SimpleConfigManager<MapConfig>,
    state: MapState,

    shader_program: ShaderProgram,
    attribute_program: AttributeProgram,
    platform_attribute: Attribute<PlatformAttr>,
}

impl Map {
    pub fn new(config_watcher: &mut ConfigWatcher, physics_sim: &mut PhysicsSimulation) -> StatusOr<Map> {
        let config_manager = SimpleConfigManager::new(config_watcher, "map.conf")?;

        let state = {
            let config = config_manager.get();
            let map_body = MapBody::new(config, physics_sim.registrar(), physics_sim.get_world_mut());
            MapState::new(config.clone(), map_body)
        };

        let vertex = file::util::resource_path("shaders", "platform_vert.glsl");
        let geometry = file::util::resource_path("shaders", "platform_geo.glsl");
        let fragment = file::util::resource_path("shaders", "platform_frag.glsl");
        let shader_program = ShaderProgram::from_long_pipeline(&vertex, &geometry, &fragment)?;
        let mut attribute_program_builder = AttributeProgram::new();
        let platform_attribute = attribute_program_builder.add_attribute();
        let attribute_program = attribute_program_builder.build();

        Ok(Map {
            config_manager,
            state,
            shader_program,
            attribute_program,
            platform_attribute
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

    pub fn draw(&mut self, projection_view: &glm::Mat4) {
        self.platform_attribute.data = self.state.body.platform_body.data_setter.get_fixture_iterator().map(|fixture| -> PlatformAttr {
            let mut polygon =  liquidfun::box2d::collision::shapes::polygon_shape::from_shape(fixture.get_shape());
            let vertices: Vec<liquidfun::box2d::common::math::Vec2> = polygon.get_vertex_iterator().collect();
            let (min_vertex, max_vertex) = (vertices[0], vertices[2]);
            PlatformAttr {
                bottom_left: glm::vec2(min_vertex.x, min_vertex.y),
                top_right: glm::vec2(max_vertex.x, max_vertex.y)
            }}).collect();

        self.shader_program.activate();
        self.shader_program.set_mat4("projection_view", projection_view);
        self.attribute_program.activate();
        self.platform_attribute.prepare_buffer();
        unsafe {
            gl::DrawArraysInstanced(gl::POINTS, 0, 4, self.platform_attribute.data.len() as GLsizei);
        }
        self.attribute_program.deactivate();
        self.shader_program.deactivate();
    }
}