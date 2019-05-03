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
        PointLight,
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

    pub fn populate_lights(&self, config: &EnemyConfig, generator_state: &EnemyGeneratorState, lights: &mut Vec<PointLight>) {
        match self {
            EnemyGeneratorStateMachine::ReadyToGenerate| EnemyGeneratorStateMachine::Cooldown(_) => {
                if let Some(position) = generator_state.position() {
                    let position = glm::vec3(position.x as f32, config.generator_light_elevation, -position.y as f32);
                    lights.push(PointLight {
                        position,
                        color: glm::vec3(config.generator_light_color.0, config.generator_light_color.1, config.generator_light_color.2),
                        attenuation: glm::vec3(config.generator_light_attenuation.0, config.generator_light_attenuation.1, config.generator_light_attenuation.2),
                    });
                }
            },
            EnemyGeneratorStateMachine::Dead => {},
        }
    }

    pub fn queue_draw(&self, config: &EnemyConfig, generator_state: &EnemyGeneratorState, sprite_renderer: &mut LightDependentSpriteRenderer) {
        let health_frac = generator_state.health().amount() as f64 / config.generator_starting_health as f64;
        let frame = match self {
            EnemyGeneratorStateMachine::Dead => config.generator_num_sprite_frames - 1,
            _ => ((1.0 - health_frac) * (config.generator_num_sprite_frames - 1) as f64).floor() as usize,
        };

        if let Some(position) = generator_state.position() {
            let world_half_size = glm::vec2(config.generator_physical_radius as f32, config.generator_physical_radius as f32) * config.generator_render_scale;
            let world_center_position = glm::vec3(position.x as f32, world_half_size.y, -position.y as f32);

            sprite_renderer.queue(vec![LightDependentSpriteData {
                world_center_position,
                world_half_size,
                sprite_frame_id: SpriteSheetFrameId {
                    name: String::from("enemy_generator.png"),
                    sprite_sheet: NamedSpriteSheet::SpriteSheet1,
                },
                frame,
                rotation: 0.0,
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