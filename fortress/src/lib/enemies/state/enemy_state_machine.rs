use crate::{
    audio::{
        AudioPlayer,
        Sound,
    },
    dimensions::{
        Attack,
        Reverse,
        time::{
            DeltaTime,
            Microseconds,
        }
    },
    enemies::{
        EnemyConfig,
        EnemyState,
        state::EnemyBody,
    },
    items::{
        ItemPickup,
        ItemSystem,
        ItemType,
    },
    particles::{
        ParticleEvent,
        ParticleSystem,
    },
    physics::PhysicsSimulation,
    render::{
        EasingFn,
        LightDependentSpriteData,
        LightDependentSpriteRenderer,
        NamedSpriteSheet,
        PointLight,
        SpriteSheetFrameId,
    },
};
use glm;
use nalgebra::{
    Point2,
    Vector2,
};

pub enum EnemyStateMachine {
    Base(EnemyBody, Microseconds),
    Dying(Option<Point2<f64>>, Microseconds),
    Dead
}

impl EnemyStateMachine {
    pub fn new(body: EnemyBody) -> EnemyStateMachine {
        EnemyStateMachine::Base(body, 0)
    }

    pub fn pre_update(&mut self, config: &EnemyConfig, dt: DeltaTime, player_locs: &Vec<Point2<f64>>, enemy_state: &mut EnemyState) -> Option<EnemyStateMachine> {
        enemy_state.pre_update(dt);

        match self {
            EnemyStateMachine::Base(body, time_elapsed) => {
                *time_elapsed += dt.as_microseconds();
                body.move_to_target(config, player_locs);
                if let Some(direction) = body.velocity() {
                    enemy_state.set_facing_dir(direction);
                }
            },
            EnemyStateMachine::Dying(_, time_elapsed) => {
                *time_elapsed += dt.as_microseconds();
            },
            _ => {},
        }
        None
    }

    pub fn take_attack(&self, config: &EnemyConfig, attack: Attack, enemy_state: &mut EnemyState, particles: &mut ParticleSystem) {
        if let EnemyStateMachine::Base(body, _) = self {
            enemy_state.take_attack(attack);
            if let Some(position) = body.position() {
                particles.queue_event(ParticleEvent {
                    position,
                    color: glm::vec3(config.enemy_hit_particle_color.0, config.enemy_hit_particle_color.1, config.enemy_hit_particle_color.2),
                    radius: config.enemy_hit_particle_radius,
                });
            }
        }
    }

    pub fn post_update(&mut self, config: &EnemyConfig, audio: &AudioPlayer, enemy_state: &EnemyState, items: &mut ItemSystem, physics_sim: &mut PhysicsSimulation) -> Option<EnemyStateMachine> {
        match self {
            EnemyStateMachine::Base(body, _) if !enemy_state.health().alive() => {
                audio.play_sound(Sound::Slash);
                let position = body.position();
                Some(EnemyStateMachine::Dying(position, 0))
            },
            EnemyStateMachine::Dying(position, time_elapsed) if *time_elapsed >= config.enemy_dying_duration_micros => {
                if let Some(position) = position {
                    let item_pickup = ItemPickup::new(ItemType::Skull, enemy_state.facing_dir());
                    items.spawn_item(item_pickup, position.clone(), physics_sim);
                }
                Some(EnemyStateMachine::Dead)
            },
            _ => None
        }
    }

    pub fn populate_lights(&self, config: &EnemyConfig, enemy_state: &EnemyState, lights: &mut Vec<PointLight>) {
        if enemy_state.age() < config.enemy_light_duration_micros {
            if let EnemyStateMachine::Base(body, _) = self {
                if let Some(position) = body.position() {
                    let age_frac = (config.enemy_light_duration_micros - enemy_state.age()) as f32 / config.enemy_light_duration_micros as f32;
                    let glow_strength = EasingFn::ease_out_quad(age_frac);

                    let position = glm::vec3(position.x as f32, config.enemy_light_elevation, -position.y as f32);
                    lights.push(PointLight {
                        position,
                        color: glm::vec3(config.enemy_light_color.0, config.enemy_light_color.1, config.enemy_light_color.2) * glow_strength,
                        attenuation: glm::vec3(config.enemy_light_attenuation.0, config.enemy_light_attenuation.1, config.enemy_light_attenuation.2),
                    });
                }
            }
        }
    }

    pub fn queue_draw(&self, config: &EnemyConfig, enemy_state: &EnemyState, sprite_renderer: &mut LightDependentSpriteRenderer) {
        let image_name = match self {
            EnemyStateMachine::Dying(_, _) => String::from("enemy1_dying.png"),
            _ => String::from("enemy1.png")
        };

        let frame = match self {
            EnemyStateMachine::Base(_, time_elapsed) => (*time_elapsed / config.enemy_walk_frame_duration_micros) as usize,
            EnemyStateMachine::Dying(_, time_elapsed) => (*time_elapsed / config.enemy_dying_frame_duration_micros) as usize,
            _ => 0,
        };

        let reverse = if enemy_state.facing_dir().is_left() {
            Reverse::none()
        } else {
            Reverse::horizontally()
        };

        if let Some(position) = self.position() {
            let world_half_size = glm::vec2(config.enemy_physical_radius as f32, config.enemy_physical_radius as f32) * config.enemy_render_scale;
            let world_center_position = glm::vec3(position.x as f32, world_half_size.y, -position.y as f32);

            sprite_renderer.queue(LightDependentSpriteData {
                world_center_position,
                world_half_size,
                sprite_frame_id: SpriteSheetFrameId {
                    name: image_name,
                    sprite_sheet: NamedSpriteSheet::SpriteSheet1,
                },
                frame,
                unit_world_rotation: Vector2::new(0.0, 0.0),
                reverse,
            });
        }
    }

    pub fn dead(&self) -> bool {
        if let EnemyStateMachine::Dead = self {
            true
        } else {
            false
        }
    }

    fn position(&self) -> Option<Point2<f64>> {
        match self {
            EnemyStateMachine::Base(body, _) => body.position(),
            EnemyStateMachine::Dying(position, _) => *position,
            _ => None
        }
    }
}
