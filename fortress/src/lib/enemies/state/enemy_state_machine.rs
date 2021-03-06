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
        DamageTextWriter,
        EnemySystemConfig,
        EnemyConfig,
        EnemyState,
        state::EnemyBody,
    },
    items::{
        ItemPickup,
        ItemSystem,
        ItemType,
        types::SkullType,
    },
    particles::{
        ParticleEvent,
        ParticleSystem,
    },
    physics::PhysicsSimulation,
    render::{
        LightDependentSpriteData,
        LightDependentSpriteRenderer,
        NamedSpriteSheet,
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
    pub fn new(body: EnemyBody) -> Self {
        Self::Base(body, 0)
    }

    pub fn pre_update(&mut self,
                      config: &EnemyConfig,
                      dt: DeltaTime,
                      player_locs: &Vec<Point2<f64>>,
                      enemy_state: &mut EnemyState) -> Option<Self> {
        match self {
            Self::Base(body, time_elapsed) => {
                *time_elapsed += dt.as_microseconds();
                body.move_to_target(config, player_locs);
                if let Some(direction) = body.velocity() {
                    enemy_state.set_facing_dir(direction);
                }
            },
            Self::Dying(_, time_elapsed) => {
                *time_elapsed += dt.as_microseconds();
            },
            _ => {},
        }
        None
    }

    pub fn take_attack(&self,
                       config: &EnemySystemConfig,
                       attack: Attack,
                       bullet_direction: Option<Vector2<f64>>,
                       enemy_state: &mut EnemyState,
                       particles: &mut ParticleSystem,
                       damage_text: &mut DamageTextWriter) {
        if let Self::Base(body, _) = self {
            let damage = attack.damage;
            enemy_state.take_attack(attack);
            if let Some(position) = body.position() {
                let blood_color = glm::vec3(config.enemy.blood_color.0, config.enemy.blood_color.1, config.enemy.blood_color.2);
                let blood_event = ParticleEvent::blood(position.clone(), blood_color, config.enemy.num_blood_particles_per_hit);
                particles.queue_event(blood_event);
                damage_text.add_damage(&config.damage_text, damage, position, bullet_direction);
            }
        }
    }

    pub fn post_update(&mut self,
                       config: &EnemyConfig,
                       audio: &AudioPlayer,
                       enemy_state: &EnemyState,
                       items: &mut ItemSystem,
                       physics_sim: &mut PhysicsSimulation) -> Option<Self> {
        match self {
            Self::Base(body, _) if !enemy_state.health().alive() => {
                audio.play_sound(Sound::EnemyKilled);
                let position = body.position();
                Some(Self::Dying(position, 0))
            },
            Self::Dying(position, time_elapsed) if *time_elapsed >= config.dying_duration_micros => {
                if let Some(position) = position {
                    let item_pickup = ItemPickup::new(ItemType::Skull(SkullType::Regular), enemy_state.facing_dir());
                    items.spawn_item(item_pickup, position.clone(), physics_sim);
                }
                Some(Self::Dead)
            },
            _ => None
        }
    }

    pub fn queue_draw(&self, config: &EnemyConfig, enemy_state: &EnemyState, sprite_renderer: &mut LightDependentSpriteRenderer) {
        let image_name = match self {
            Self::Dying(_, _) => String::from("enemy1_dying.png"),
            _ => String::from("enemy1.png")
        };

        let frame = match self {
            Self::Base(_, time_elapsed) => (*time_elapsed / config.walk_frame_duration_micros) as usize,
            Self::Dying(_, time_elapsed) => (*time_elapsed / config.dying_frame_duration_micros) as usize,
            _ => 0,
        };

        let reverse = if enemy_state.facing_dir().is_left() {
            Reverse::none()
        } else {
            Reverse::horizontally()
        };

        if let Some(position) = self.position() {
            let world_half_size = glm::vec2(config.physical_radius as f32, config.physical_radius as f32) * config.render_scale;
            let world_center_position = glm::vec3(position.x as f32, world_half_size.y, -position.y as f32);

            sprite_renderer.queue(LightDependentSpriteData {
                world_center_position,
                world_half_size,
                sprite_frame_id: SpriteSheetFrameId::new(image_name, NamedSpriteSheet::SpriteSheet1),
                frame,
                unit_world_rotation: Vector2::new(0.0, 0.0),
                reverse,
            });
        }
    }

    pub fn dead(&self) -> bool {
        match self {
            Self::Dead => true,
            _ => false,
        }
    }

    fn position(&self) -> Option<Point2<f64>> {
        match self {
            Self::Base(body, _) => body.position(),
            Self::Dying(position, _) => *position,
            _ => None
        }
    }
}
