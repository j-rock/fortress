use crate::{
    audio::{
        AudioPlayer,
        Sound,
    },
    dimensions::{
        Attack,
        Health
    },
    enemies::{
        EnemyGeneratorConfig,
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
    pub fn new(config: &EnemyGeneratorConfig, body: EnemyGeneratorBody) -> EnemyGeneratorState {
        EnemyGeneratorState {
            body,
            health: Health::new(config.starting_health),
        }
    }

    pub fn take_attack(&mut self, config: &EnemyGeneratorConfig, audio: &AudioPlayer, attack: Attack, particles: &mut ParticleSystem) {
        self.health.withdraw(attack.damage);
        if self.health.alive() {
            audio.play_sound(Sound::EnemyGeneratorHurt);
        } else {
            audio.play_sound(Sound::EnemyGeneratorKilled);
        }

        if let Some(position) = self.position() {
            let blood_color = glm::vec3(config.blood_color.0, config.blood_color.1, config.blood_color.2);
            let blood_event = ParticleEvent::blood(position, blood_color, config.num_blood_particles_per_hit);
            particles.queue_event(blood_event);
        }
    }

    pub fn health(&self) -> Health {
        self.health
    }

    pub fn position(&self) -> Option<Point2<f64>> {
        self.body.position()
    }

    pub fn compute_spawn(&self, config: &EnemyGeneratorConfig) -> Option<Point2<f64>> {
        let position = self.body.position()?;
        let orientation = self.body.orientation();
        let offset = config.offset_distance * Vector2::new(orientation.cos(), orientation.sin());
        Some(Point2::new(position.x + offset.x, position.y + offset.y))
    }

    pub fn orientation(&self) -> f64 {
        self.body.orientation()
    }
}
