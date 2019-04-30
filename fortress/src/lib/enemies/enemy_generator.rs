use crate::{
    audio::AudioPlayer,
    dimensions::{
        Attack,
        time::DeltaTime
    },
    enemies::{
        Enemy,
        EnemyConfig,
        EnemyGeneratorId,
        state::{
            EnemyGeneratorBody,
            EnemyGeneratorState,
            EnemyGeneratorStateMachine,
        }
    },
    physics::PhysicsSimulation,
    render::LightDependentSpriteRenderer,
};
use generational_slab::Slab;

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
    pub fn new(config: &EnemyConfig, generator_id: EnemyGeneratorId, spawn: EnemyGeneratorSpawn, physics_sim: &mut PhysicsSimulation) -> EnemyGenerator {
        let body = EnemyGeneratorBody::new(config, generator_id, spawn, physics_sim);
        EnemyGenerator {
            generator_state: EnemyGeneratorState::new(config, body),
            generator_state_machine: EnemyGeneratorStateMachine::default(),
        }
    }

    pub fn pre_update(&mut self, config: &EnemyConfig, dt: DeltaTime, enemies: &mut Slab<Enemy>, physics_sim: &mut PhysicsSimulation) {
        if let Some(state) = self.generator_state_machine.pre_update(config, dt, &self.generator_state, enemies, physics_sim) {
            self.generator_state_machine = state;
        }
    }

    pub fn post_update(&mut self, audio: &AudioPlayer) {
        if let Some(state) = self.generator_state_machine.post_update(audio, &mut self.generator_state) {
            self.generator_state_machine = state;
        }
    }

    pub fn queue_draw(&self, config: &EnemyConfig, sprite_renderer: &mut LightDependentSpriteRenderer) {
        self.generator_state_machine.queue_draw(config, &self.generator_state, sprite_renderer);
    }

    pub fn take_attack(&mut self, attack: Attack) {
        self.generator_state_machine.take_attack(attack, &mut self.generator_state);
    }
}