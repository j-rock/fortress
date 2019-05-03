use crate::{
    dimensions::{
        Attack,
        Health,
        time::{
            DeltaTime,
            Microseconds
        },
    },
    enemies::EnemyConfig,
};
use nalgebra::Vector2;

pub struct EnemyState {
    health: Health,
    age: Microseconds,
    facing_dir: Option<Vector2<f64>>,
}

impl EnemyState {
    pub fn new(config: &EnemyConfig) -> EnemyState {
        EnemyState {
            health: Health::new(config.enemy_starting_health),
            age: 0,
            facing_dir: None,
        }
    }

    pub fn pre_update(&mut self, dt: DeltaTime) {
        self.age += dt.as_microseconds();
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

    pub fn facing_dir(&self) -> Option<Vector2<f64>> {
        self.facing_dir
    }

    pub fn set_facing_dir(&mut self, dir: Vector2<f64>) {
        self.facing_dir = Some(dir);
    }
}
