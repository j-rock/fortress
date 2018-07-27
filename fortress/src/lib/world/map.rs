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
use physics::PhysicsSimulation;
use render::{
    attribute,
    AttributeProgram,
    Attribute,
    ShaderProgram,
};

#[derive(Debug, Deserialize)]
struct Platform {
    top_left_x: i32,
    top_left_y: i32,
    width: i32,
    height: i32
}

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

#[derive(Deserialize)]
struct MapConfig {
    platforms: Vec<Platform>,
}

pub struct Map {
    config_manager: SimpleConfigManager<MapConfig>,
    platform_body: liquidfun::box2d::dynamics::body::Body,
    shader_program: ShaderProgram,
    attribute_program: AttributeProgram,
    platform_attribute: Attribute<PlatformAttr>,
}

impl Map {
    pub fn new(config_watcher: &mut ConfigWatcher, physics_sim: &mut PhysicsSimulation) -> StatusOr<Map> {
        let config_manager = SimpleConfigManager::new(config_watcher, "map.conf")?;
        let platform_body = Self::create_body_from_platforms(config_manager.get(), physics_sim.get_world_mut());

        let vertex = file::util::resource_path("shaders", "platform_vert.glsl");
        let geometry = file::util::resource_path("shaders", "platform_geo.glsl");
        let fragment = file::util::resource_path("shaders", "platform_frag.glsl");
        let shader_program = ShaderProgram::from_long_pipeline(&vertex, &geometry, &fragment)?;
        let mut attribute_program = AttributeProgram::new();
        let platform_attribute = attribute_program.add_attribute();
        attribute_program.done_adding_attributes();

        Ok(Map {
            config_manager,
            platform_body,
            shader_program,
            attribute_program,
            platform_attribute
        })
    }

    pub fn update(&mut self) {
        if self.config_manager.update() {
            self.redeploy_platforms();
        }
    }

    fn redeploy_platforms(&mut self) {
        let mut world = self.platform_body.get_world();
        world.destroy_body(&mut self.platform_body);
        self.platform_body = Self::create_body_from_platforms(self.config_manager.get(), &mut world);
    }

    fn create_body_from_platforms(config: &MapConfig, world: &mut liquidfun::box2d::dynamics::world::World) -> liquidfun::box2d::dynamics::body::Body {
        let body_def = liquidfun::box2d::dynamics::body::BodyDef::default();
        let platform_body = world.create_body(&body_def);
        let mut poly_shape = liquidfun::box2d::collision::shapes::polygon_shape::PolygonShape::new();
        for platform in config.platforms.iter() {
            let (hx, hy) = (platform.width as f32 / 2.0, platform.height as f32 / 2.0);
            let center = liquidfun::box2d::common::math::Vec2::new(platform.top_left_x as f32 + hx, platform.top_left_y as f32 - hy);
            poly_shape.set_as_box_oriented(hx, hy, &center, 0.0);
            let fixture_def = liquidfun::box2d::dynamics::fixture::FixtureDef::new(&poly_shape);
            platform_body.create_fixture(&fixture_def);
        }
        platform_body
    }

    pub fn draw(&mut self, projection_view: &glm::Mat4) {
        // Need to add platform_body fixtures to platform_attribute.data.
        self.platform_attribute.data = self.platform_body.get_fixture_iterator().map(|fixture| -> PlatformAttr {
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