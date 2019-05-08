use crate::{
    audio::AudioPlayer,
    dimensions::{
        Attack,
        time::DeltaTime
    },
    enemies::{
        EnemyConfig,
        EnemyId,
        state::{
            EnemyBody,
            EnemyState,
            EnemyStateMachine,
        }
    },
    items::ItemSystem,
    physics::PhysicsSimulation,
    render::{
        LightDependentSpriteRenderer,
        PointLight,
    },
};
use nalgebra::Point2;

pub struct Enemy {
    enemy_state: EnemyState,
    enemy_state_machine: EnemyStateMachine,
}

impl Enemy {
    pub fn new(config: &EnemyConfig, enemy_id: EnemyId, spawn: Point2<f64>, physics_sim: &mut PhysicsSimulation) -> Enemy {
        let enemy_state = EnemyState::new(config);
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

    pub fn populate_lights(&self, config: &EnemyConfig, lights: &mut Vec<PointLight>) {
        self.enemy_state_machine.populate_lights(config, &self.enemy_state, lights);
    }

    pub fn queue_draw(&self, config: &EnemyConfig, sprite_renderer: &mut LightDependentSpriteRenderer) {
        self.enemy_state_machine.queue_draw(config, &self.enemy_state, sprite_renderer);
    }

    pub fn take_attack(&mut self, attack: Attack) {
        self.enemy_state_machine.take_attack(attack, &mut self.enemy_state);
    }

    pub fn dead(&self) -> bool {
        self.enemy_state_machine.dead()
    }

    pub fn redeploy(&mut self, config: &EnemyConfig, enemy_id: EnemyId, spawn: Point2<f64>, physics_sim: &mut PhysicsSimulation) {
        self.enemy_state = EnemyState::new(config);
        let enemy_body = EnemyBody::new(config, enemy_id, spawn, physics_sim);
        self.enemy_state_machine = EnemyStateMachine::new(enemy_body);
    }
}
