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
    BoxData,
    BoxRenderer,
};
use wraith::Wraith;

pub struct Player {
    config_manager: SimpleConfigManager<PlayerConfig>,
    registrar: EntityRegistrar,

    player_state: PlayerState,
    player_state_machine: Box<dyn PlayerStateMachine>,
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

        Ok(Player {
            config_manager,
            registrar,
            player_state,
            player_state_machine,
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

    pub fn draw(&self, box_renderer: &mut BoxRenderer) {
        let (body_position, body_size) = (self.player_state.get_body_position(), self.player_state.config.size);
        let (sword_position, sword_size) = (self.player_state.get_sword_position(), self.player_state.config.sword_sensor_size);

        let boxes = vec!(
            BoxData {
                position: glm::vec2(body_position.x, body_position.y),
                half_size: glm::vec2(body_size.0 as f32 / 2.0, body_size.1 as f32 / 2.0),
                rgba_tl: glm::vec4(0.8, 0.3, 0.3, 0.0),
                rgba_tr: glm::vec4(0.4, 0.0, 0.8, 0.0),
                rgba_bl: glm::vec4(0.2, 0.2, 1.0, 0.0),
                rgba_br: glm::vec4(1.0, 0.0, 0.1, 0.0),
            },
            BoxData {
                position: glm::vec2(sword_position.x, sword_position.y),
                half_size: glm::vec2(sword_size.0 as f32 / 2.0, sword_size.1 as f32 / 2.0),
                rgba_tl: glm::vec4(0.8, 0.3, 0.3, 0.0),
                rgba_tr: glm::vec4(0.4, 0.0, 0.8, 0.0),
                rgba_bl: glm::vec4(0.2, 0.2, 1.0, 0.0),
                rgba_br: glm::vec4(1.0, 0.0, 0.1, 0.0),
            });

        box_renderer.queue(boxes.as_slice());
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
