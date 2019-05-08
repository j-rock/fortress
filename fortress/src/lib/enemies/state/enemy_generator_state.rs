use crate::{
    dimensions::{
        Attack,
        Health
    },
    enemies::{
        EnemyConfig,
        state::EnemyGeneratorBody
    },
};
use nalgebra::{
    Point2,
    Vector2,
};

pub struct EnemyGeneratorState {
    body: EnemyGeneratorBody,
    health: Health,
}

impl EnemyGeneratorState {
    pub fn new(config: &EnemyConfig, body: EnemyGeneratorBody) -> EnemyGeneratorState {
        EnemyGeneratorState {
            body,
            health: Health::new(config.generator_starting_health),
        }
    }

    pub fn take_attack(&mut self, attack: Attack) {
        self.health.withdraw(attack.damage);
    }

    pub fn health(&self) -> Health {
        self.health
    }

    pub fn position(&self) -> Option<Point2<f64>> {
        self.body.position()
    }

    pub fn compute_spawn(&self, config: &EnemyConfig) -> Option<Point2<f64>> {
        let position = self.body.position()?;
        let orientation = self.body.orientation();
        let offset = config.generator_offset_distance * Vector2::new(orientation.cos(), orientation.sin());
        Some(Point2::new(position.x + offset.x, position.y + offset.y))
    }

    pub fn orientation(&self) -> f64 {
        self.body.orientation()
    }
}
