use crate::{
    audio::AudioPlayer,
    control::{
        Controller,
        ControllerId,
    },
    dimensions::{
        Attack,
        time::DeltaTime
    },
    physics::PhysicsSimulation,
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
        FullyIlluminatedSpriteRenderer,
        LightDependentSpriteRenderer,
        PointLight,
    },
    weapons::BulletId,
};
use nalgebra::Point2;

pub struct Player {
    player_state: PlayerState,
    player_state_machine: Box<dyn PlayerStateMachine>,
}

impl Player {
    pub fn new(config: &PlayerConfig, player_id: PlayerId, spawn: Point2<f64>, physics_sim: &mut PhysicsSimulation) -> Player {
        let player_state = PlayerState::new(player_id, config, spawn, physics_sim);
        let player_state_machine = Box::new(PlayerUpright::new());

        Player {
            player_state,
            player_state_machine,
        }
    }

    pub fn pre_update(&mut self, audio: &AudioPlayer, controller_id: ControllerId, controller: &Controller, dt: DeltaTime) {
        self.player_state.pre_update(dt);

        if let Some(player_state_machine) = self.player_state_machine.pre_update(&mut self.player_state, audio, controller_id, controller, dt) {
            self.player_state_machine = player_state_machine;
        }
    }

    pub fn post_update(&mut self, audio: &AudioPlayer) {
        self.player_state.post_update();

        if let Some(player_state_machine) = self.player_state_machine.post_update(&self.player_state, audio) {
            self.player_state_machine = player_state_machine;
        }
    }

    pub fn redeploy(&mut self, config: &PlayerConfig, physics_sim: &mut PhysicsSimulation) {
        self.player_state.redeploy(config, physics_sim);
        self.player_state_machine = Box::new(PlayerUpright::new());
    }

    pub fn respawn(&mut self, spawn: Point2<f64>) {
        self.player_state.respawn(spawn);
    }

    pub fn get_player_id(&self) -> PlayerId {
        self.player_state.get_player_id()
    }

    pub fn populate_lights(&self, config: &PlayerConfig, lights: &mut Vec<PointLight>) {
        self.player_state.populate_lights(config, lights);
    }

    pub fn queue_draw(&self, config: &PlayerConfig, full_light: &mut FullyIlluminatedSpriteRenderer, light_dependent: &mut LightDependentSpriteRenderer) {
        self.player_state.queue_draw(config, full_light, light_dependent);
    }

    pub fn bullet_hit(&mut self, bullet_id: BulletId) {
        self.player_state.bullet_hit(bullet_id);
    }

    pub fn bullet_attack(&self, bullet_id: BulletId) -> Option<Attack> {
        self.player_state.bullet_attack(bullet_id)
    }

    pub fn position(&self) -> Option<Point2<f64>> {
        self.player_state.position()
    }
}
