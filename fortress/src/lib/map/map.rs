use app::StatusOr;
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
    MapPhysics,
};
use physics::PhysicsSimulation;
use render::{
    attribute,
    AttributeProgram,
    Attribute,
    FragmentShader,
    GeometryShader,
    ShaderProgram,
    VertexShader,
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
    map_physics: MapPhysics,
    shader_program: ShaderProgram<Attribute<PlatformAttr>>,
}

impl Map {
    pub fn new(config_watcher: &mut ConfigWatcher, physics_sim: &mut PhysicsSimulation) -> StatusOr<Map> {
        let config_manager = SimpleConfigManager::new(config_watcher, "map.conf")?;
        let map_physics = MapPhysics::new(config_manager.get(), physics_sim.registrar(), physics_sim);

        let mut attribute_program_builder = AttributeProgram::new();
        let platform_attribute = attribute_program_builder.add_attribute();
        let attribute_program = attribute_program_builder.build(platform_attribute);

        let vertex = VertexShader::new(&file::util::resource_path("shaders", "platform_vert.glsl"))?;
        let geometry = GeometryShader::new(&file::util::resource_path("shaders", "platform_geo.glsl"))?;
        let fragment = FragmentShader::new(&file::util::resource_path("shaders", "platform_frag.glsl"))?;
        let shader_program = ShaderProgram::from_long_pipeline(attribute_program, &vertex, &geometry, &fragment)?;

        Ok(Map {
            config_manager,
            map_physics,
            shader_program,
        })
    }

    pub fn register(&mut self) {
        let data: *const Map = self as *const Map;
        self.map_physics.register(data);
    }

    pub fn update(&mut self) {
        if self.config_manager.update() {
            self.map_physics.redeploy(self.config_manager.get());
        }
    }

    pub fn draw(&mut self, projection_view: &glm::Mat4) {
        self.shader_program.activate();
        self.shader_program.set_mat4("projection_view", projection_view);

        {
            let attributes = self.shader_program.attributes_mut();
            attributes.data = self.map_physics.get_platform_body_mut().get_fixture_iterator().map(|fixture| -> PlatformAttr {
                let mut polygon = liquidfun::box2d::collision::shapes::polygon_shape::from_shape(fixture.get_shape());
                let vertices: Vec<liquidfun::box2d::common::math::Vec2> = polygon.get_vertex_iterator().collect();
                let (min_vertex, max_vertex) = (vertices[0], vertices[2]);
                PlatformAttr {
                    bottom_left: glm::vec2(min_vertex.x, min_vertex.y),
                    top_right: glm::vec2(max_vertex.x, max_vertex.y)
                }
            }).collect();

            attributes.prepare_buffer();
        }

        unsafe {
            gl::DrawArraysInstanced(gl::POINTS, 0, 4, self.platform_attribute.data.len() as GLsizei);
        }
        self.shader_program.deactivate();
    }
}