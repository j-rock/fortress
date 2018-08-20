use app::StatusOr;
use control::{
    Controller,
    ControlEvent::RespawnEntities,
};
use dimensions::{
    time::DeltaTime
};
use entity::{
    EntityType,
    EntityRegistrar,
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
use physics::{
    CollisionMatcher,
    PhysicsSimulation,
};
use player::{
    PlayerConfig,
    PlayerState,
    state::{
        PlayerStateMachine,
        PlayerUpright,
    }
};
use render::{
    attribute,
    Attribute,
    AttributeProgram,
    ShaderProgram,
};
use wraith::Wraith;

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
    registrar: EntityRegistrar,

    player_state: PlayerState,
    player_state_machine: Box<dyn PlayerStateMachine>,

    shader_program: ShaderProgram,
    attribute_program: AttributeProgram,
    player_attribute: Attribute<PlayerAttr>,
}

impl Player {
    pub fn new(config_watcher: &mut ConfigWatcher, physics_sim: &mut PhysicsSimulation) -> StatusOr<Player> {
        let config_manager = SimpleConfigManager::new(config_watcher, "player.conf")?;
        let registrar = physics_sim.registrar();

        let (player_state, player_state_machine) = {
            let config: &PlayerConfig = config_manager.get();
            let player_state = PlayerState::new(config.clone(), &registrar, physics_sim.get_world_mut());
            let player_state_machine = Box::new(PlayerUpright::new());

            (player_state, player_state_machine)
        };

        let vertex = file::util::resource_path("shaders", "player_vert.glsl");
        let geometry = file::util::resource_path("shaders", "player_geo.glsl");
        let fragment = file::util::resource_path("shaders", "player_frag.glsl");
        let shader_program = ShaderProgram::from_long_pipeline(&vertex, &geometry, &fragment)?;
        let mut attribute_program_builder = AttributeProgram::new();
        let player_attribute = attribute_program_builder.add_attribute();
        let attribute_program = attribute_program_builder.build();

        Ok(Player {
            config_manager,
            registrar,
            player_state,
            player_state_machine,
            shader_program,
            attribute_program,
            player_attribute
        })
    }

    pub fn register(&mut self) {
        let player: *const Player = self as *const Player;
        self.player_state.register(player);
    }

    pub fn pre_update(&mut self, controller: &Controller, dt: DeltaTime) {
        if self.config_manager.update() || controller.just_pressed(RespawnEntities) {
            self.redeploy();
        }

        self.player_state.pre_update(dt);

        if let Some(player_state_machine) = self.player_state_machine.pre_update(&mut self.player_state, controller, dt) {
            self.player_state_machine = player_state_machine;
        }
    }

    pub fn post_update(&mut self) {
        if let Some(player_state_machine) = self.player_state_machine.post_update() {
            self.player_state_machine = player_state_machine;
        }
    }

    fn redeploy(&mut self) {
        {
            let config = self.config_manager.get();
            let mut world = self.player_state.body.body.get_world();
            self.player_state = PlayerState::new(config.clone(), &self.registrar, &mut world);
            self.player_state_machine = Box::new(PlayerUpright::new());
        }

        self.register();
    }

    pub fn draw(&mut self, projection_view: &glm::Mat4) {
        let (body_position, body_size) = (self.player_state.get_body_position(), self.player_state.config.size);
        let (sword_position, sword_size) = (self.player_state.get_sword_position(), self.player_state.config.sword_sensor_size);
        self.player_attribute.data = vec!(
            PlayerAttr {
                position: glm::vec2(body_position.x, body_position.y),
                half_size: glm::vec2(body_size.0 as f32 / 2.0, body_size.1 as f32 / 2.0)
            },
            PlayerAttr {
                position: glm::vec2(sword_position.x, sword_position.y),
                half_size: glm::vec2(sword_size.0 as f32 / 2.0, sword_size.1 as f32 / 2.0)
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

    pub fn foot_sensor_hit_something() -> CollisionMatcher {
        CollisionMatcher::match_one(EntityType::PlayerFootSensor, Box::new(|entity| {
            let player: &mut Self = entity.resolve();
            player.player_state_machine.make_foot_contact();
        }))
    }

    pub fn slash_wraith() -> CollisionMatcher {
        CollisionMatcher::match_two(EntityType::PlayerSwordSensor, EntityType::Wraith, Box::new(|sword_ent, wraith_ent| {
            let player: &Self = sword_ent.resolve();
            let wraith: &mut Wraith = wraith_ent.resolve();
            wraith.take_slashing(player.player_state.body.facing_dir);
        }))
    }
}
