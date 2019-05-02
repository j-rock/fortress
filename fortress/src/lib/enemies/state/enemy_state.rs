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

pub struct EnemyState {
    health: Health,
    age: Microseconds,
}

impl EnemyState {
    pub fn new(config: &EnemyConfig) -> EnemyState {
        EnemyState {
            health: Health::new(config.enemy_starting_health),
            age: 0,
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
}
