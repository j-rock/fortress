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
    items::ItemSystem,
    physics::PhysicsSimulation,
    render::{
        LightDependentSpriteRenderer,
        PointLight,
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
    pub fn new(config: &EnemyConfig, generator_id: EnemyGeneratorId, spawn: EnemyGeneratorSpawn, physics_sim: &mut PhysicsSimulation) -> EnemyGenerator {
        let body = EnemyGeneratorBody::new(config, generator_id, spawn, physics_sim);
        EnemyGenerator {
            generator_state: EnemyGeneratorState::new(config, body),
            generator_state_machine: EnemyGeneratorStateMachine::default(),
        }
    }

    pub fn pre_update(&mut self, config: &EnemyConfig, dt: DeltaTime, player_locs: &Vec<Point2<f64>>, enemies: &mut Slab<Enemy>, physics_sim: &mut PhysicsSimulation) {
        if let Some(state) = self.generator_state_machine.pre_update(config, dt, &self.generator_state, player_locs, enemies, physics_sim) {
            self.generator_state_machine = state;
        }
    }

    pub fn post_update(&mut self, audio: &AudioPlayer, items: &mut ItemSystem, physics_sim: &mut PhysicsSimulation) {
        if let Some(state) = self.generator_state_machine.post_update(audio, &mut self.generator_state, items, physics_sim) {
            self.generator_state_machine = state;
        }
    }

    pub fn point_light(&self, config: &EnemyConfig) -> Option<PointLight> {
        self.generator_state_machine.point_light(config, &self.generator_state)
    }

    pub fn queue_draw(&self, config: &EnemyConfig, sprite_renderer: &mut LightDependentSpriteRenderer) {
        self.generator_state_machine.queue_draw(config, &self.generator_state, sprite_renderer);
    }

    pub fn dead(&self) -> bool {
        self.generator_state_machine.dead()
    }

    pub fn take_attack(&mut self, attack: Attack) {
        self.generator_state_machine.take_attack(attack, &mut self.generator_state);
    }
}