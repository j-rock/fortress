use app::StatusOr;
use control::{
    Controller,
    ControlEvent::PlayerJump,
    ControlEvent::PlayerMove,
    ControlEvent::PlayerRespawn,
};
use dimensions::{
    LrDirection,
    time::DeltaTime,
};
use entity::EntityRegistrar;
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
use physics::PhysicsSimulation;
use player::{
    PlayerConfig,
    PlayerPhysics,
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
    player_physics: PlayerPhysics,

    shader_program: ShaderProgram,
    attribute_program: AttributeProgram,
    player_attribute: Attribute<PlayerAttr>,
}

impl Player {
    pub fn new(config_watcher: &mut ConfigWatcher, physics_sim: &mut PhysicsSimulation) -> StatusOr<Player> {
        let config_manager = SimpleConfigManager::new(config_watcher, "player.conf")?;
        let player_physics = PlayerPhysics::new(config_manager.get(), physics_sim);

        let vertex = file::util::resource_path("shaders", "player_vert.glsl");
        let geometry = file::util::resource_path("shaders", "player_geo.glsl");
        let fragment = file::util::resource_path("shaders", "player_frag.glsl");
        let shader_program = ShaderProgram::from_long_pipeline(&vertex, &geometry, &fragment)?;
        let mut attribute_program = AttributeProgram::new();
        let player_attribute = attribute_program.add_attribute();
        attribute_program.done_adding_attributes();

        Ok(Player {
            config_manager,
            player_physics,
            shader_program,
            attribute_program,
            player_attribute
        })
    }

    pub fn update(&mut self, registrar: &mut EntityRegistrar, controller: &Controller, dt: DeltaTime) {
        if self.config_manager.update() || controller.just_pressed(PlayerRespawn) {
            self.redeploy_physics(registrar);
        }

        let data: *const Player = self as *const Player;
        self.player_physics.update(dt, registrar, data);

        let speed = self.config_manager.get().move_speed;
        if controller.is_pressed(PlayerMove(LrDirection::Left)) {
            self.player_physics.move_horizontal(speed, Some(LrDirection::Left));
        } else if controller.is_pressed(PlayerMove(LrDirection::Right)) {
            self.player_physics.move_horizontal(speed, Some(LrDirection::Right));
        } else {
            self.player_physics.move_horizontal(speed, None)
        }

        if controller.just_pressed(PlayerJump) {
           self.player_physics.jump();
        }
    }

    pub fn make_foot_contact(&mut self) {
        self.player_physics.make_foot_contact();
    }

    fn redeploy_physics(&mut self, registrar: &mut EntityRegistrar) {
        let config = self.config_manager.get();
        self.player_physics.redeploy(config, registrar);
    }

    pub fn draw(&mut self, projection_view: &glm::Mat4) {
        let position = self.player_physics.get_position();
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
