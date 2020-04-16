use crate::{
    audio::AudioPlayer,
    dimensions::{
        Attack,
        time::DeltaTime
    },
    enemies::{
        Enemy,
        EnemyGeneratorConfig,
        EnemyGeneratorId,
        EnemySystemConfig,
        state::{
            EnemyGeneratorBody,
            EnemyGeneratorState,
            EnemyGeneratorStateMachine,
        }
    },
    items::ItemSystem,
    particles::ParticleSystem,
    physics::PhysicsSimulation,
    render::{
        LightDependentSpriteRenderer,
        PointLight,
        ScreenShake,
    },
};
use generational_slab::Slab;
use nalgebra::Point2;

#[derive(Copy, Clone, Deserialize)]
pub struct EnemyGeneratorSpawn {
    pub position: (f64, f64),
    pub orientation: f64,
}

pub struct EnemyGenerator {
    generator_state: EnemyGeneratorState,
    generator_state_machine: EnemyGeneratorStateMachine,
}

impl EnemyGenerator {
    pub fn new(config: &EnemyGeneratorConfig, generator_id: EnemyGeneratorId, spawn: EnemyGeneratorSpawn, physics_sim: &mut PhysicsSimulation) -> EnemyGenerator {
        let body = EnemyGeneratorBody::new(config, generator_id, spawn, physics_sim);
        EnemyGenerator {
            generator_state: EnemyGeneratorState::new(config, generator_id, body),
            generator_state_machine: EnemyGeneratorStateMachine::default(),
        }
    }

    pub fn pre_update(&mut self,
                      config: &EnemySystemConfig,
                      dt: DeltaTime,
                      player_locs: &Vec<Point2<f64>>,
                      enemies: &mut Slab<Enemy>,
                      physics_sim: &mut PhysicsSimulation) {
        self.generator_state_machine
            .pre_update(config, player_locs, dt, &mut self.generator_state, enemies, physics_sim)
            .map(|state| {
                self.generator_state_machine = state;
            });
    }

    pub fn post_update(&mut self, config: &EnemyGeneratorConfig, items: &mut ItemSystem, shake: &mut ScreenShake, physics_sim: &mut PhysicsSimulation) {
        if let Some(state) = self.generator_state_machine.post_update(config, &mut self.generator_state, items, shake, physics_sim) {
            self.generator_state_machine = state;
        }
    }

    pub fn point_light(&self, config: &EnemyGeneratorConfig) -> Option<PointLight> {
        self.generator_state_machine.point_light(config, &self.generator_state)
    }

    pub fn queue_draw(&self, config: &EnemyGeneratorConfig, sprite_renderer: &mut LightDependentSpriteRenderer) {
        self.generator_state_machine.queue_draw(config, &self.generator_state, sprite_renderer);
    }

    pub fn dead(&self) -> bool {
        self.generator_state_machine.dead()
    }

    pub fn take_attack(&mut self, config: &EnemyGeneratorConfig, audio: &AudioPlayer, attack: Attack, particles: &mut ParticleSystem) {
        self.generator_state_machine.take_attack(config, audio, attack, &mut self.generator_state, particles);
    }

    pub fn tally_killed_enemy(&mut self) {
        self.generator_state.tally_killed_enemy();
    }
}