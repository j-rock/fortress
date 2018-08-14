use app::StatusOr;
use control::{
    Controller,
    ControlEvent::PlayerRespawn,
};
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
use physics::PhysicsSimulation;
use player::{
    PlayerConfig,
    PlayerState,
    states::{
        PlayerBody,
        PlayerStateMachine,
        PlayerUpright,
        SlashState,
    }
};
use render::{
    attribute,
    Attribute,
    AttributeProgram,
    FragmentShader,
    GeometryShader,
    ShaderProgram,
    VertexShader,
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
    player_state: PlayerState,
    player_state_machine: Box<dyn PlayerStateMachine>,
    shader_program: ShaderProgram<Attribute<PlayerAttr>>,
}

impl Player {
    pub fn new(config_watcher: &mut ConfigWatcher, physics_sim: &mut PhysicsSimulation) -> StatusOr<Player> {
        let config_manager = SimpleConfigManager::new(config_watcher, "player.conf")?;

        let (player_state, player_state_machine) = {
            let config = config_manager.get();
            let player_body = PlayerBody::new(config, physics_sim.registrar(), physics_sim.get_world_mut());
            let player_state = PlayerState::new(config.clone(), player_body);
            let slash_state = SlashState::new(config);
            let player_state_machine = Box::new(PlayerUpright::new(slash_state));

            (player_state, player_state_machine)
        };

        let mut attribute_program_builder = AttributeProgram::new();
        let player_attribute = attribute_program_builder.add_attribute();
        let attribute_program = attribute_program_builder.build(player_attribute);

        let vertex = VertexShader::new(&file::util::resource_path("shaders", "player_vert.glsl"))?;
        let geometry = GeometryShader::new(&file::util::resource_path("shaders", "player_geo.glsl"))?;
        let fragment = FragmentShader::new(&file::util::resource_path("shaders", "player_frag.glsl"))?;
        let shader_program = ShaderProgram::from_long_pipeline(attribute_program, &vertex, &geometry, &fragment)?;

        Ok(Player {
            config_manager,
            player_state,
            player_state_machine,
            shader_program,
        })
    }

    pub fn register(&mut self) {
        let player: *const Player = self as *const Player;
        self.player_state.register(player);
    }

    pub fn pre_update(&mut self, controller: &Controller, dt: DeltaTime) {
        if self.config_manager.update() || controller.just_pressed(PlayerRespawn) {
            self.redeploy();
        }

        if let Some(player_state_machine) = self.player_state_machine.pre_update(&mut self.player_state, controller, dt) {
            self.player_state_machine = player_state_machine;
        }
    }

    pub fn post_update(&mut self) {
        if let Some(player_state_machine) = self.player_state_machine.post_update() {
            self.player_state_machine = player_state_machine;
        }
    }

    pub fn make_foot_contact(&mut self) {
        self.player_state_machine.make_foot_contact();
    }

    fn redeploy(&mut self) {
        {
            let config = self.config_manager.get();
            let mut world = self.player_state.body.body.get_world();
            let player_body = PlayerBody::new(config, self.player_state.body.foot_sensor.registrar.clone(), &mut world);
            let slash_state = SlashState::new(config);
            self.player_state = PlayerState::new(config.clone(), player_body);
            self.player_state_machine = Box::new(PlayerUpright::new(slash_state));
        }

        self.register();
    }

    pub fn draw(&mut self, projection_view: &glm::Mat4) {
        self.shader_program.activate();
        self.shader_program.set_mat4("projection_view", projection_view);
        {
            let (body_position, body_size) = (self.player_state.get_body_position(), self.player_state.config.size);
            let (sword_position, sword_size) = (self.player_state.get_sword_position(), self.player_state.config.sword_sensor_size);
            let attributes = self.shader_program.attributes_mut();
            attributes.data = vec!(
                PlayerAttr {
                    position: glm::vec2(body_position.x, body_position.y),
                    half_size: glm::vec2(body_size.0 as f32 / 2.0, body_size.1 as f32 / 2.0)
                },
                PlayerAttr {
                    position: glm::vec2(sword_position.x, sword_position.y),
                    half_size: glm::vec2(sword_size.0 as f32 / 2.0, sword_size.1 as f32 / 2.0)
                });

            attributes.prepare_buffer();
        }
        unsafe {
            gl::DrawArraysInstanced(gl::POINTS, 0, 4, self.player_attribute.data.len() as GLsizei);
        }
        self.shader_program.deactivate();
    }
}
