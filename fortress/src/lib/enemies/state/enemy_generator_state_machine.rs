use crate::{
    audio::AudioPlayer,
    dimensions::{
        Attack,
        time::{
            DeltaTime,
            Microseconds,
        },
    },
    enemies::{
        Enemy,
        EnemyConfig,
        EnemyId,
        state::EnemyGeneratorState,
    },
    physics::PhysicsSimulation,
    render::{
        LightDependentSpriteData,
        LightDependentSpriteRenderer,
        NamedSpriteSheet,
        SpriteSheetFrameId,
    },
};
use generational_slab::Slab;

pub enum EnemyGeneratorStateMachine {
    ReadyToGenerate,
    Cooldown(Microseconds),
    Dead
}

impl Default for EnemyGeneratorStateMachine {
    fn default() -> EnemyGeneratorStateMachine {
        EnemyGeneratorStateMachine::ReadyToGenerate
    }
}

impl EnemyGeneratorStateMachine {
    pub fn pre_update(&self, config: &EnemyConfig, dt: DeltaTime, generator_state: &EnemyGeneratorState, enemies: &mut Slab<Enemy>, physics_sim: &mut PhysicsSimulation) -> Option<EnemyGeneratorStateMachine> {
        match self {
            EnemyGeneratorStateMachine::ReadyToGenerate => {
                Self::new_enemy(config, generator_state, enemies, physics_sim);
                Some(EnemyGeneratorStateMachine::Cooldown(config.generator_cooldown_duration_micros))
            },
            EnemyGeneratorStateMachine::Cooldown(time_left) => {
                Some(EnemyGeneratorStateMachine::Cooldown(time_left - dt.as_microseconds()))
            },
            _ => None,
        }
    }

    pub fn post_update(&self, _audio: &AudioPlayer, generator_state: &mut EnemyGeneratorState) -> Option<EnemyGeneratorStateMachine> {
        match self {
           EnemyGeneratorStateMachine::ReadyToGenerate | EnemyGeneratorStateMachine::Cooldown(_) if !generator_state.health().alive() => {
               generator_state.stop_interacting_physically();
               Some(EnemyGeneratorStateMachine::Dead)
           },
           EnemyGeneratorStateMachine::Cooldown(time_elapsed) if *time_elapsed <= 0 => {
               Some(EnemyGeneratorStateMachine::ReadyToGenerate)
           },
           _ => None
       }
    }

    pub fn queue_draw(&self, config: &EnemyConfig, generator_state: &EnemyGeneratorState, sprite_renderer: &mut LightDependentSpriteRenderer) {
        let image_name = match self {
            EnemyGeneratorStateMachine::Dead => String::from("enemy_generator_dead.png"),
            _ => String::from("enemy_generator.png")
        };

        if let Some(position) = generator_state.position() {
            let world_bottom_center_position = glm::vec3(position.x as f32, 0.0, -position.y as f32);
            let world_half_size = glm::vec2(config.generator_physical_radius as f32, config.generator_physical_radius as f32);

            sprite_renderer.queue(vec![LightDependentSpriteData {
                world_bottom_center_position,
                world_half_size,
                sprite_frame_id: SpriteSheetFrameId {
                    name: image_name,
                    sprite_sheet: NamedSpriteSheet::SpriteSheet1,
                },
                frame: 0,
            }]);
        }
    }

    pub fn take_attack(&self, attack: Attack, generator_state: &mut EnemyGeneratorState) {
        generator_state.take_attack(attack);
    }

    fn new_enemy(config: &EnemyConfig, generator_state: &EnemyGeneratorState, enemies: &mut Slab<Enemy>, physics_sim: &mut PhysicsSimulation) {
        if let Some(spawn) = generator_state.compute_spawn(config) {
            let enemy_entry = enemies.vacant_entry();
            let enemy_id = EnemyId::from_key(enemy_entry.key());
            let enemy = Enemy::new(config, enemy_id, spawn, physics_sim);
            enemy_entry.insert(enemy);
        }
    }
}