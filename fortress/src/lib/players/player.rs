use crate::{
    app::RandGen,
    audio::AudioPlayer,
    control::IdentifiedController,
    dimensions::{
        Attack,
        time::DeltaTime
    },
    items::ItemPickup,
    particles::ParticleSystem,
    physics::PhysicsSimulation,
    players::{
        PlayerSystemConfig,
        PlayerId,
        state::{
            PlayerState,
            PlayerStateMachine,
        }
    },
    render::{
        FullyIlluminatedSpriteRenderer,
        LightDependentSpriteRenderer,
        PointLights,
    },
    weapons::BulletId,
};
use nalgebra::Point2;

pub struct Player {
    player_state: PlayerState,
    player_state_machine: PlayerStateMachine,
}

impl Player {
    pub fn new(config: &PlayerSystemConfig, player_id: PlayerId, spawn: Point2<f64>, physics_sim: &mut PhysicsSimulation) -> Player {
        let player_state = PlayerState::new(player_id, config, spawn, physics_sim);
        let player_state_machine = PlayerStateMachine::new();

        Player {
            player_state,
            player_state_machine,
        }
    }

    pub fn pre_update<'a>(&mut self,
                          config: &PlayerSystemConfig,
                          audio: &AudioPlayer,
                          controller: IdentifiedController<'a>,
                          dt: DeltaTime,
                          particles: &mut ParticleSystem,
                          rng: &mut RandGen) {
        if let Some(player_state_machine) = self.player_state_machine.pre_update(config, audio, controller, dt, particles, rng, &mut self.player_state) {
            self.player_state_machine = player_state_machine;
        }
    }

    pub fn post_update(&mut self) {
        if let Some(player_state_machine) = self.player_state_machine.post_update(&mut self.player_state) {
            self.player_state_machine = player_state_machine;
        }
    }

    pub fn redeploy(&mut self, config: &PlayerSystemConfig, physics_sim: &mut PhysicsSimulation) {
        self.player_state.redeploy(config, physics_sim);
        self.player_state_machine = PlayerStateMachine::new();
    }

    pub fn respawn(&mut self, spawn: Point2<f64>) {
        self.player_state.respawn(spawn);
    }

    pub fn get_player_id(&self) -> PlayerId {
        self.player_state.get_player_id()
    }

    pub fn populate_lights(&self, config: &PlayerSystemConfig, lights: &mut PointLights) {
        self.player_state_machine.populate_lights(config, &self.player_state, lights);
    }

    pub fn queue_draw(&self, config: &PlayerSystemConfig, full_light: &mut FullyIlluminatedSpriteRenderer, light_dependent: &mut LightDependentSpriteRenderer) {
        self.player_state_machine.queue_draw(config, &self.player_state, full_light, light_dependent);
    }

    pub fn bullet_hit(&mut self, bullet_id: BulletId) {
        self.player_state_machine.bullet_hit(bullet_id, &mut self.player_state);
    }

    pub fn bullet_attack(&self, bullet_id: BulletId) -> Option<Attack> {
        self.player_state_machine.bullet_attack(&self.player_state, bullet_id)
    }

    pub fn position(&self) -> Option<Point2<f64>> {
        self.player_state_machine.position(&self.player_state)
    }

    pub fn collect_item(&mut self, item_pickup: ItemPickup) {
        self.player_state_machine.collect_item(item_pickup, &mut self.player_state);
    }
}
