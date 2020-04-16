use crate::{
    audio::AudioPlayer,
    dimensions::{
        Attack,
        LrDirection,
        Reverse,
        time::{
            DeltaTime,
            Microseconds,
        },
    },
    enemies::{
        Enemy,
        EnemyGeneratorConfig,
        EnemySystemConfig,
        EnemyId,
        state::EnemyGeneratorState,
    },
    items::{
        ItemPickup,
        ItemSystem,
        ItemType,
    },
    particles::ParticleSystem,
    physics::PhysicsSimulation,
    render::{
        LightDependentSpriteData,
        LightDependentSpriteRenderer,
        NamedSpriteSheet,
        PointLight,
        ScreenShake,
        SpriteSheetFrameId,
    },
};
use generational_slab::Slab;
use nalgebra::{
    Point2,
    Vector2
};

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
    pub fn pre_update(&self, config: &EnemySystemConfig, dt: DeltaTime, generator_state: &EnemyGeneratorState, player_locs: &Vec<Point2<f64>>, enemies: &mut Slab<Enemy>, physics_sim: &mut PhysicsSimulation) -> Option<EnemyGeneratorStateMachine> {
        match self {
            EnemyGeneratorStateMachine::ReadyToGenerate => {
                Self::new_enemy(config, generator_state, player_locs, enemies, physics_sim);
                Some(EnemyGeneratorStateMachine::Cooldown(config.generator.cooldown_duration_micros))
            },
            EnemyGeneratorStateMachine::Cooldown(time_left) => {
                Some(EnemyGeneratorStateMachine::Cooldown(time_left - dt.as_microseconds()))
            },
            _ => None,
        }
    }

    pub fn post_update(&self,
                       config: &EnemyGeneratorConfig,
                       generator_state: &mut EnemyGeneratorState,
                       items: &mut ItemSystem,
                       shake: &mut ScreenShake,
                       physics_sim: &mut PhysicsSimulation) -> Option<EnemyGeneratorStateMachine> {
        match self {
           EnemyGeneratorStateMachine::ReadyToGenerate | EnemyGeneratorStateMachine::Cooldown(_) if !generator_state.health().alive() => {
               if let Some(position) = generator_state.position() {
                   let item_pickup = ItemPickup::new(ItemType::MegaSkull, LrDirection::from_radians(generator_state.orientation()));
                   items.spawn_item(item_pickup, position.clone(), physics_sim);
               }
               shake.intensify(config.death_screen_shake_intensity);
               Some(EnemyGeneratorStateMachine::Dead)
           },
           EnemyGeneratorStateMachine::Cooldown(time_elapsed) if *time_elapsed <= 0 => {
               Some(EnemyGeneratorStateMachine::ReadyToGenerate)
           },
           _ => None
       }
    }

    pub fn point_light(&self, config: &EnemyGeneratorConfig, generator_state: &EnemyGeneratorState) -> Option<PointLight> {
        match self {
            EnemyGeneratorStateMachine::ReadyToGenerate| EnemyGeneratorStateMachine::Cooldown(_) => {
                let generator_position = generator_state.position()?;
                let position =
                    glm::vec3(generator_position.x as f32, 0.0, -generator_position.y as f32) +
                    glm::vec3(config.light_offset.0, config.light_offset.1, config.light_offset.2);

                let color = glm::vec3(config.light_color.0, config.light_color.1, config.light_color.2);
                let attenuation = glm::vec3(config.light_attenuation.0, config.light_attenuation.1, config.light_attenuation.2);
                Some(PointLight::new(position, color, attenuation))
            },
            EnemyGeneratorStateMachine::Dead => None,
        }
    }

    pub fn queue_draw(&self, config: &EnemyGeneratorConfig, generator_state: &EnemyGeneratorState, sprite_renderer: &mut LightDependentSpriteRenderer) {
        let health_frac = generator_state.health().amount() as f64 / config.starting_health as f64;
        let frame = match self {
            EnemyGeneratorStateMachine::Dead => config.num_sprite_frames - 1,
            _ => ((1.0 - health_frac) * (config.num_sprite_frames) as f64).floor() as usize,
        };

        if let Some(position) = generator_state.position() {
            let world_half_size = glm::vec2(config.physical_radius as f32, config.physical_radius as f32) * config.render_scale;
            let world_center_position = glm::vec3(position.x as f32, world_half_size.y, -position.y as f32);

            sprite_renderer.queue(LightDependentSpriteData {
                world_center_position,
                world_half_size,
                sprite_frame_id: SpriteSheetFrameId::new(String::from("enemy_generator.png"), NamedSpriteSheet::SpriteSheet1),
                frame,
                unit_world_rotation: Vector2::new(0.0, 0.0),
                reverse: Reverse::none(),
            });
        }
    }

    pub fn take_attack(&self, config: &EnemyGeneratorConfig, audio: &AudioPlayer, attack: Attack, generator_state: &mut EnemyGeneratorState, particles: &mut ParticleSystem) {
        generator_state.take_attack(config, audio, attack, particles);
    }

    pub fn dead(&self) -> bool {
        match self {
            EnemyGeneratorStateMachine::Dead => true,
            _ => false,
        }
    }

    fn new_enemy(config: &EnemySystemConfig, generator_state: &EnemyGeneratorState, player_locs: &Vec<Point2<f64>>, enemies: &mut Slab<Enemy>, physics_sim: &mut PhysicsSimulation) {
        if let Some(position) = generator_state.position() {
            player_locs
                .iter()
                .min_by_key(|player_loc| {
                    let diff = position - **player_loc;
                    (diff.x * diff.x + diff.y * diff.y).round() as i64
                })
                .and_then(|closest_player_loc| -> Option<()> {
                    let displacement = *closest_player_loc - position;
                    let distance = displacement.norm();
                    if distance < config.generator.generate_distance {
                        if let Some(spawn) = generator_state.compute_spawn(&config.generator) {
                            let enemy_entry = enemies.vacant_entry();
                            let enemy_id = EnemyId::from_key(enemy_entry.key());
                            let enemy = Enemy::new(&config.enemy, enemy_id, spawn, physics_sim);
                            enemy_entry.insert(enemy);
                        }
                    }
                    None
                });
        }
    }
}