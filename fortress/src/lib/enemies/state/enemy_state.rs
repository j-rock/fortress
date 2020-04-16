use crate::{
    dimensions::{
        Attack,
        Health,
        LrDirection,
        time::{
            DeltaTime,
            Microseconds
        },
    },
    enemies::{
        EnemyConfig,
        EnemyGeneratorId,
    },
};
use nalgebra::Vector2;

pub struct EnemyState {
    generator_id: EnemyGeneratorId,
    health: Health,
    age: Microseconds,
    facing_dir: LrDirection,
}

impl EnemyState {
    pub fn new(config: &EnemyConfig, generator_id: EnemyGeneratorId) -> EnemyState {
        EnemyState {
            generator_id,
            health: Health::new(config.starting_health),
            age: 0,
            facing_dir: LrDirection::Right,
        }
    }

    pub fn pre_update(&mut self, dt: DeltaTime) {
        self.age += dt.as_microseconds();
    }

    pub fn generator_id(&self) -> EnemyGeneratorId {
        self.generator_id
    }

    pub fn health(&self) -> Health {
        self.health
    }

    pub fn age(&self) -> Microseconds {
        self.age
    }

    pub fn take_attack(&mut self, attack: Attack) {
        self.health.withdraw(attack.damage);
    }

    pub fn facing_dir(&self) -> LrDirection {
        self.facing_dir
    }

    pub fn set_facing_dir(&mut self, dir: Vector2<f64>) {
        if dir.x < 0.0 {
            self.facing_dir = LrDirection::Left;
        }
        if dir.x > 0.0 {
            self.facing_dir = LrDirection::Right;
        }
    }
}
