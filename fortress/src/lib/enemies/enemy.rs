use crate::{
    audio::AudioPlayer,
    dimensions::{
        Attack,
        time::DeltaTime
    },
    enemies::{
        EnemyConfig,
        EnemyGeneratorId,
        EnemyId,
        state::{
            EnemyBody,
            EnemyState,
            EnemyStateMachine,
        }
    },
    items::ItemSystem,
    particles::ParticleSystem,
    physics::PhysicsSimulation,
    render::LightDependentSpriteRenderer,
    world::DamageTextWriter,
};
use nalgebra::{
    Point2,
    Vector2,
};

pub struct Enemy {
    enemy_state: EnemyState,
    enemy_state_machine: EnemyStateMachine,
}

impl Enemy {
    pub fn new(config: &EnemyConfig, enemy_id: EnemyId, generator_id: EnemyGeneratorId, spawn: Point2<f64>, physics_sim: &mut PhysicsSimulation) -> Enemy {
        let enemy_state = EnemyState::new(config, generator_id);
        let enemy_body = EnemyBody::new(config, enemy_id, spawn, physics_sim);
        let enemy_state_machine = EnemyStateMachine::new(enemy_body);

        Enemy {
            enemy_state,
            enemy_state_machine
        }
    }

    pub fn pre_update(&mut self, config: &EnemyConfig, dt: DeltaTime, player_locs: &Vec<Point2<f64>>) {
        if let Some(enemy_state_machine) = self.enemy_state_machine.pre_update(config, dt, player_locs, &mut self.enemy_state) {
            self.enemy_state_machine = enemy_state_machine;
        }
    }

    pub fn post_update(&mut self, config: &EnemyConfig, audio: &AudioPlayer, items: &mut ItemSystem, physics_sim: &mut PhysicsSimulation) {
        if let Some(enemy_state_machine) = self.enemy_state_machine.post_update(config, audio, &self.enemy_state, items, physics_sim) {
            self.enemy_state_machine = enemy_state_machine;
        }
    }

    pub fn queue_draw(&self, config: &EnemyConfig, sprite_renderer: &mut LightDependentSpriteRenderer) {
        self.enemy_state_machine.queue_draw(config, &self.enemy_state, sprite_renderer);
    }

    pub fn take_attack(&mut self,
                       config: &EnemyConfig,
                       attack: Attack,
                       bullet_direction: Option<Vector2<f64>>,
                       particles: &mut ParticleSystem,
                       damage_text: &mut DamageTextWriter) {
        self.enemy_state_machine.take_attack(config, attack, bullet_direction, &mut self.enemy_state, particles, damage_text);
    }

    pub fn dead(&self) -> bool {
        self.enemy_state_machine.dead()
    }

    pub fn generator_id(&self) -> EnemyGeneratorId {
        self.enemy_state.generator_id()
    }
}
