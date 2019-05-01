use crate::{
    dimensions::{
        Attack,
        Health,
    },
    enemies::EnemyConfig,
};

pub struct EnemyState {
    health: Health,
}

impl EnemyState {
    pub fn new(config: &EnemyConfig) -> EnemyState {
        EnemyState {
            health: Health::new(config.enemy_starting_health),
        }
    }

    pub fn health(&self) -> Health {
        self.health
    }

    pub fn take_attack(&mut self, attack: Attack) {
        self.health.withdraw(attack.damage);
    }
}
