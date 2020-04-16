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
        EnemyGeneratorId,
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
    id: EnemyGeneratorId,
    body: EnemyGeneratorBody,
    health: Health,
    live_spawned_enemy_count: usize,
}

impl EnemyGeneratorState {
    pub fn new(config: &EnemyGeneratorConfig, id: EnemyGeneratorId, body: EnemyGeneratorBody) -> EnemyGeneratorState {
        EnemyGeneratorState {
            id,
            body,
            health: Health::new(config.starting_health),
            live_spawned_enemy_count: 0,
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

    pub fn id(&self) -> EnemyGeneratorId {
        self.id
    }

    pub fn health(&self) -> Health {
        self.health
    }

    pub fn position(&self) -> Option<Point2<f64>> {
        self.body.position()
    }

    pub fn live_spawned_enemy_count(&self) -> usize {
        self.live_spawned_enemy_count
    }

    pub fn tally_spawned_enemy(&mut self) {
        self.live_spawned_enemy_count += 1;
    }

    pub fn tally_killed_enemy(&mut self) {
        if self.live_spawned_enemy_count > 0 {
            self.live_spawned_enemy_count -= 1;
        }
    }

    pub fn compute_spawn(&self, config: &EnemyGeneratorConfig) -> Option<Point2<f64>> {
        let position = self.body.position()?;
        let orientation = self.body.orientation();
        let offset = config.spawn_offset_distance * Vector2::new(orientation.cos(), orientation.sin());
        Some(Point2::new(position.x + offset.x, position.y + offset.y))
    }

    pub fn orientation(&self) -> f64 {
        self.body.orientation()
    }
}
