use app::StatusOr;
use controls::{
    Controller,
    ControlEvent::PlayerJump,
    ControlEvent::PlayerMove,
    ControlEvent::PlayerRespawn,
};
use dimensions::{
    LrDirection,
    time::DeltaTime,
};
use entity::{
    EntityRegistrar,
    EntityType,
    Registered,
};
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
use player::{
    JumpTracker,
    PlayerConfig,
};
use render::{
    attribute,
    Attribute,
    AttributeProgram,
    ShaderProgram,
};

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
    foot_sensor: Registered<liquidfun::box2d::dynamics::fixture::Fixture>,
    jump_tracker: JumpTracker,

    shader_program: ShaderProgram,
    attribute_program: AttributeProgram,
    player_attribute: Attribute<PlayerAttr>,
}

impl Player {
    pub fn new(config_watcher: &mut ConfigWatcher, physics_sim: &mut PhysicsSimulation) -> StatusOr<Player> {
        let config_manager = SimpleConfigManager::new(config_watcher, "player.conf")?;
        let (player_body, foot_sensor, jump_tracker) = {
            let config = config_manager.get();
            let (player_body, foot_sensor) = Self::create_body_from_config(config, physics_sim.get_world_mut());
            let foot_sensor = Registered::new(foot_sensor, EntityType::PlayerFootSensor);
            let jump_tracker = JumpTracker::new(config);
            (player_body, foot_sensor, jump_tracker)
        };

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
            foot_sensor,
            jump_tracker,
            shader_program,
            attribute_program,
            player_attribute
        })
    }

    pub fn update(&mut self, registrar: &mut EntityRegistrar, controller: &Controller, dt: DeltaTime) {
        if self.config_manager.update() || controller.just_pressed(PlayerRespawn) {
            self.redeploy_player_body(registrar);
        }

        let data: *const Player = self as *const Player;
        self.foot_sensor.register::<Player>(registrar, data);

        self.jump_tracker.update(dt);

        if controller.is_pressed(PlayerMove(LrDirection::Left)) {
            self.move_horizontal(Some(LrDirection::Left));
        } else if controller.is_pressed(PlayerMove(LrDirection::Right)) {
            self.move_horizontal(Some(LrDirection::Right));
        } else {
            self.move_horizontal(None)
        }

        if controller.just_pressed(PlayerJump) {
           self.jump();
        }
    }

    fn move_horizontal(&mut self, dir: Option<LrDirection>) {
        let player_speed = self.config_manager.get().player_speed;
        let desired_horizontal_velocity = player_speed * match dir {
            None => 0.0,
            Some(LrDirection::Left) => -1.0,
            Some(LrDirection::Right) => 1.0
        };

        let actual_body_velocity = *self.player_body.get_linear_velocity();
        let mass = self.player_body.get_mass();
        let impulse = liquidfun::box2d::common::math::Vec2::new(mass * (desired_horizontal_velocity - actual_body_velocity.x), 0.0);
        let body_center = *self.player_body.get_world_center();
        self.player_body.apply_linear_impulse(&impulse, &body_center, true);
    }

    fn jump(&mut self) {
        self.jump_tracker.try_jump(&self.player_body);
    }

    pub fn make_foot_contact(&mut self) {
        self.jump_tracker.make_foot_contact();
    }

    fn redeploy_player_body(&mut self, registrar: &mut EntityRegistrar) {
        self.foot_sensor.unregister(registrar);

        let mut world = self.player_body.get_world();
        world.destroy_body(&mut self.player_body);

        let config = self.config_manager.get();
        let (player_body, foot_sensor_fixture) = Self::create_body_from_config(config, &mut world);
        self.player_body = player_body;
        self.foot_sensor = Registered::new(foot_sensor_fixture, EntityType::PlayerFootSensor);
        self.jump_tracker = JumpTracker::new(config);
    }

    fn create_body_from_config(config: &PlayerConfig, world: &mut liquidfun::box2d::dynamics::world::World)
        -> (liquidfun::box2d::dynamics::body::Body, liquidfun::box2d::dynamics::fixture::Fixture) {
        let mut body_def = liquidfun::box2d::dynamics::body::BodyDef::default();
        body_def.body_type = liquidfun::box2d::dynamics::body::BodyType::DynamicBody;
        body_def.position = liquidfun::box2d::common::math::Vec2::new(config.spawn_location.0 as f32, config.spawn_location.1 as f32);
        body_def.fixed_rotation = true;

        let player_body = world.create_body(&body_def);

        // Player body fixture
        let mut poly_shape = liquidfun::box2d::collision::shapes::polygon_shape::PolygonShape::new();
        let (hx, hy) = (config.size.0 as f32 / 2.0, config.size.1 as f32 / 2.0);
        poly_shape.set_as_box(hx, hy);

        let mut fixture_def = liquidfun::box2d::dynamics::fixture::FixtureDef::new(&poly_shape);
        fixture_def.restitution = config.restitution;
        fixture_def.filter.category_bits = 0x0002;
        player_body.create_fixture(&fixture_def);

        // Foot sensor fixture
        let (hx, hy) = (config.foot_sensor_size.0 / 2.0, config.foot_sensor_size.1 / 2.0);
        let sensor_center = liquidfun::box2d::common::math::Vec2::new(config.foot_sensor_center.0, config.foot_sensor_center.1);
        poly_shape.set_as_box_oriented(hx, hy, &sensor_center, 0.0);
        fixture_def.filter.category_bits = 0x0001;
        fixture_def.filter.mask_bits = 0xFFFF & !0x0002; // Ignore player body.
        fixture_def.is_sensor = true;
        let foot_sensor = player_body.create_fixture(&fixture_def);

        (player_body, foot_sensor)
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
