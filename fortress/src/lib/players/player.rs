use crate::{
    audio::AudioPlayer,
    buffs::Buff,
    control::{
        Controller,
        ControllerId,
    },
    dimensions::{
        Attack,
        time::DeltaTime
    },
    entities::EntityType,
    physics::{
        CollisionMatcher,
        PhysicsSimulation,
    },
    players::{
        PlayerConfig,
        PlayerId,
        PlayerState,
        state::{
            PlayerStateMachine,
            PlayerUpright,
        }
    },
    render::{
        BoxData,
        BoxRenderer,
    },
    wraiths::Wraith
};
use glm;
use liquidfun::box2d::common::math::Vec2;

pub struct Player {
    player_state: PlayerState,
    player_state_machine: Box<dyn PlayerStateMachine>,
}

impl Player {
    pub fn new(config: &PlayerConfig, player_id: PlayerId, spawn: Vec2, physics_sim: &mut PhysicsSimulation) -> Player {
        let player_state = PlayerState::new(player_id, config.clone(), spawn, physics_sim);
        let player_state_machine = Box::new(PlayerUpright::new());

        Player {
            player_state,
            player_state_machine,
        }
    }

    pub fn register(&mut self) {
        let player: *const Player = self as *const Player;
        self.player_state.register(player);
    }

    pub fn pre_update(&mut self, audio: &AudioPlayer, controller_id: ControllerId, controller: &Controller, dt: DeltaTime) {
        self.player_state.pre_update(dt);

        if let Some(player_state_machine) = self.player_state_machine.pre_update(&mut self.player_state, audio, controller_id, controller, dt) {
            self.player_state_machine = player_state_machine;
        }
    }

    pub fn post_update(&mut self, audio: &AudioPlayer) {
        if let Some(player_state_machine) = self.player_state_machine.post_update(&self.player_state, audio) {
            self.player_state_machine = player_state_machine;
        }
    }

    pub fn absorb_buff(&mut self, buff: Buff) {
        self.player_state.absorb_buff(buff);
    }

    pub fn get_position(&self) -> glm::Vec2 {
        let body_pos = self.player_state.get_body_position();
        glm::vec2(body_pos.x, body_pos.y)
    }

    pub fn redeploy(&mut self, config: &PlayerConfig, physics_sim: &mut PhysicsSimulation) {
        self.player_state = PlayerState::new(self.player_state.player_id, config.clone(), self.player_state.spawn, physics_sim);
        self.player_state_machine = Box::new(PlayerUpright::new());

        self.register();
    }

    pub fn get_player_id(&self) -> PlayerId {
        self.player_state.player_id
    }

    pub fn respawn(&mut self, spawn: Vec2) {
        self.player_state.respawn(spawn);
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

        self.player_state.crossbow.draw(box_renderer);
    }

    pub fn slash_wraith() -> CollisionMatcher {
        CollisionMatcher::match_two(EntityType::PlayerSwordSensor, EntityType::Wraith, Box::new(|_audio, sword_ent, wraith_ent| {
            let player: &Self = sword_ent.resolve();
            let wraith: &mut Wraith = wraith_ent.resolve();
            let attack = Attack {
                damage: player.player_state.sword.get_damage(),
                knockback_strength: player.player_state.sword.get_knockback_strength(),
                knockback_dir: player.player_state.body.facing_dir
            };
            wraith.take_attack(attack);
        }))
    }
}