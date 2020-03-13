use crate::{
    dimensions::{
        Attack,
        Health
    },
    enemies::{
        EnemyConfig,
        state::EnemyGeneratorBody
    },
    particles::{
        ParticleEvent,
        ParticleSystem,
    }
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

    pub fn take_attack(&mut self, config: &EnemyConfig, attack: Attack, particles: &mut ParticleSystem) {
        self.health.withdraw(attack.damage);
        if let Some(position) = self.position() {
            let blood_color = glm::vec3(config.generator_blood_color.0, config.generator_blood_color.1, config.generator_blood_color.2);
            let blood_event = ParticleEvent::blood(position, blood_color, config.generator_num_blood_particles_per_hit);
            particles.queue_event(blood_event);
        }
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
