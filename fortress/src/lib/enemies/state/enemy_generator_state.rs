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

enum Either<LeftT, RightT> {
    Left(LeftT),
    Right(RightT)
}

pub struct EnemyGeneratorState {
    body: Either<EnemyGeneratorBody, Option<Point2<f64>>>,
    health: Health,
}

impl EnemyGeneratorState {
    pub fn new(config: &EnemyConfig, body: EnemyGeneratorBody) -> EnemyGeneratorState {
        EnemyGeneratorState {
            body: Either::Left(body),
            health: Health::new(config.generator_starting_health),
        }
    }

    pub fn take_attack(&mut self, attack: Attack) {
        self.health.withdraw(attack.damage);
    }

    pub fn stop_interacting_physically(&mut self) {
        if let Either::Left(ref body) = self.body {
            self.body = Either::Right(body.position());
        }
    }

    pub fn health(&self) -> Health {
        self.health
    }

    pub fn position(&self) -> Option<Point2<f64>> {
        match self.body {
            Either::Left(ref body) => body.position(),
            Either::Right(ref pos) => *pos,
        }
    }

    pub fn compute_spawn(&self, config: &EnemyConfig) -> Option<Point2<f64>> {
        match self.body {
            Either::Right(_) => None,
            Either::Left(ref body) => {
                let position = body.position()?;
                let orientation = body.orientation();
                let offset = config.generator_offset_distance * Vector2::new(orientation.cos(), orientation.sin());
                Some(Point2::new(position.x + offset.x, position.y + offset.y))
            }
        }
    }
}
