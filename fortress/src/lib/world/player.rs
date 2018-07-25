use app::StatusOr;
use controls::{
    Controller,
    ControlEvent::PlayerMove,
    ControlEvent::PlayerRespawn,
};
use dimensions::LrDirection;
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
use render::{
    attribute,
    Attribute,
    AttributeProgram,
    ShaderProgram,
};
use world::PhysicsSimulation;

#[derive(Deserialize)]
struct PlayerConfig {
    size: (i32, i32),
    spawn_location: (i32, i32),
    player_speed: f32,
    // Between [0, 1]
    restitution: f32,
}

#[repr(C)]
struct PlayerAttr {
    position: glm::Vec2,
    half_size: glm::Vec2,
}

impl attribute::KnownComponent for PlayerAttr {
    fn component() -> (attribute::NumComponents, attribute::ComponentType) {
        (attribute::NumComponents::S4, attribute::ComponentType::FLOAT)
    }
}

pub struct Player {
    config_manager: SimpleConfigManager<PlayerConfig>,
    player_body: liquidfun::box2d::dynamics::body::Body,

    shader_program: ShaderProgram,
    attribute_program: AttributeProgram,
    player_attribute: Attribute<PlayerAttr>,
}

impl Player {
    pub fn new(config_watcher: &mut ConfigWatcher, physics_sim: &mut PhysicsSimulation) -> StatusOr<Player> {
        let config_manager = SimpleConfigManager::new(config_watcher, "player.conf")?;
        let player_body = Self::create_body_from_config(config_manager.get(), physics_sim.get_world_mut());

        let vertex = file::util::resource_path("shaders", "player_vert.glsl");
        let geometry = file::util::resource_path("shaders", "player_geo.glsl");
        let fragment = file::util::resource_path("shaders", "player_frag.glsl");
        let shader_program = ShaderProgram::from_long_pipeline(&vertex, &geometry, &fragment)?;
        let mut attribute_program = AttributeProgram::new();
        let player_attribute = attribute_program.add_attribute();
        attribute_program.done_adding_attributes();

        Ok(Player {
            config_manager,
            player_body,
            shader_program,
            attribute_program,
            player_attribute
        })
    }

    pub fn update(&mut self, controller: &Controller) {
        if self.config_manager.update() || controller.just_released(PlayerRespawn) {
            self.redeploy_player_body();
        }

        let desired_horizontal_velocity = if controller.is_pressed(PlayerMove(LrDirection::Left)) {
            -self.config_manager.get().player_speed
        } else if controller.is_pressed(PlayerMove(LrDirection::Right)) {
            self.config_manager.get().player_speed
        } else {
            0.0
        };

        let actual_body_velocity = *self.player_body.get_linear_velocity();
        let mass = self.player_body.get_mass();
        let impulse = liquidfun::box2d::common::math::Vec2::new(mass * (desired_horizontal_velocity - actual_body_velocity.x), 0.0);
        let body_center = *self.player_body.get_world_center();
        self.player_body.apply_linear_impulse(&impulse, &body_center, true);
    }

    fn redeploy_player_body(&mut self) {
        let mut world = self.player_body.get_world();
        world.destroy_body(&self.player_body);
        self.player_body = Self::create_body_from_config(self.config_manager.get(), &mut world);
    }

    fn create_body_from_config(config: &PlayerConfig, world: &mut liquidfun::box2d::dynamics::world::World) -> liquidfun::box2d::dynamics::body::Body {
        let mut body_def = liquidfun::box2d::dynamics::body::BodyDef::default();
        body_def.body_type = liquidfun::box2d::dynamics::body::BodyType::DynamicBody;
        body_def.position = liquidfun::box2d::common::math::Vec2::new(config.spawn_location.0 as f32, config.spawn_location.1 as f32);
        body_def.fixed_rotation = true;

        let player_body = world.create_body(&body_def);

        let mut poly_shape = liquidfun::box2d::collision::shapes::polygon_shape::PolygonShape::new();
        let (hx, hy) = (config.size.0 as f32 / 2.0, config.size.1 as f32 / 2.0);
        poly_shape.set_as_box(hx, hy);

        let mut fixture_def = liquidfun::box2d::dynamics::fixture::FixtureDef::new(&poly_shape);
        fixture_def.restitution = config.restitution;
        player_body.create_fixture(&fixture_def);
        player_body
    }

    pub fn draw(&mut self, projection_view: &glm::Mat4) {
        let position = self.player_body.get_position();
        let size = self.config_manager.get().size;
        self.player_attribute.data =
            vec!(PlayerAttr {
                position: glm::vec2(position.x, position.y),
                half_size: glm::vec2(size.0 as f32 / 2.0, size.1 as f32 / 2.0)
            });

        self.shader_program.activate();
        self.shader_program.set_mat4("projection_view", projection_view);
        self.attribute_program.activate();
        self.player_attribute.prepare_buffer();
        unsafe {
            gl::DrawArraysInstanced(gl::POINTS, 0, 4, self.player_attribute.data.len() as GLsizei);
        }
        self.attribute_program.deactivate();
        self.shader_program.deactivate();
    }
}
