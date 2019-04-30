use crate::{
    dimensions::{
        Attack,
        Health,
    },
    enemies::{
        EnemyConfig,
        state::EnemyBody
    }
};
use nalgebra::Point2;

pub struct EnemyState {
    body: EnemyBody,
    health: Health,
}

impl EnemyState {
    pub fn new(config: &EnemyConfig, body: EnemyBody) -> EnemyState {
        EnemyState {
            body,
            health: Health::new(config.enemy_starting_health),
        }
    }

    pub fn health(&self) -> Health {
        self.health
    }

    pub fn position(&self) -> Option<Point2<f64>> {
        self.body.position()
    }

    pub fn take_attack(&mut self, attack: Attack) {
        self.health.withdraw(attack.damage);
    }
}
