use crate::{
    audio::{
        AudioPlayer,
        Sound,
    },
    dimensions::{
        Attack,
        time::{
            self,
            DeltaTime,
        }
    },
    entities::Entity,
    physics::PhysicsSimulation,
    players::{
        PlayerConfig,
        PlayerId,
    },
    render::{
        NamedTexture,
        SpriteData,
        SpriteRenderer,
    },
    weapons::{
        BulletId,
        Bullet,
        WeaponStats,
    },
};
use glm;
use nalgebra::{
    Point2,
    Vector2
};
use nphysics2d::algebra::Velocity2;
use slab::Slab;

pub struct Weapon {
    stats: WeaponStats,
    bullet_radius: f64,
    bullets: Slab<Bullet>,
    current_delay: Option<time::Microseconds>,
    physics_sim: PhysicsSimulation,
}

impl Weapon {
    pub fn new(config: &PlayerConfig, physics_sim: &PhysicsSimulation) -> Weapon {
        Weapon {
            stats: WeaponStats::new(config),
            bullet_radius: config.bullet_radius,
            bullets: Slab::new(),
            current_delay: None,
            physics_sim: physics_sim.clone(),
        }
    }

    pub fn pre_update(&mut self, dt: DeltaTime) {
        if let Some(delay) = self.current_delay {
            let new_delay = delay - dt.as_microseconds();
            self.current_delay = if new_delay <= 0 {
                None
            } else {
                Some(new_delay)
            };
        }
    }

    pub fn try_fire(&mut self, audio: &AudioPlayer, player_id: PlayerId, start_position: Point2<f64>, direction: Vector2<f64>) {
        if self.current_delay.is_none() {
            self.current_delay = Some(self.stats.get_firing_period());

            let vacant_entry = self.bullets.vacant_entry();
            let bullet_id = BulletId::new(vacant_entry.key());
            let entity = Entity::Bullet(player_id, bullet_id);

            let bullet_speed = self.stats.get_bullet_speed();
            let linear_vel = bullet_speed * direction;
            let velocity = Velocity2::linear(linear_vel.x, linear_vel.y);

            let bullet = Bullet::new(entity, self.bullet_radius, start_position, velocity, &mut self.physics_sim);
            vacant_entry.insert(bullet);

            audio.play_sound(Sound::Blast);
        }
    }

    pub fn get_attack(&self, bullet_id: BulletId) -> Option<Attack> {
        self.bullets
            .get(bullet_id.to_usize())
            .and_then(|bullet| {
                bullet.get_attack(self.stats.get_bullet_damage(), self.stats.get_knockback_strength())
            })
    }

    pub fn remove_bullet(&mut self, bullet_id: BulletId) {
        self.bullets.remove(bullet_id.to_usize());
    }

    pub fn draw(&self, sprite_renderer: &mut SpriteRenderer) {
        let sprites: Vec<_> = self.bullets.iter().map(|(_idx, bullet)| -> SpriteData {
            let body_position = bullet.get_position();
            SpriteData {
                world_bottom_center_position: glm::vec3(body_position.x as f32, 0.1, -body_position.y as f32),
                world_half_size: glm::vec2(self.bullet_radius as f32, self.bullet_radius as f32),
                tex_bottom_left: glm::vec2(0.0001, 0.0001),
                tex_top_right: glm::vec2(0.9999, 0.9999),
            }
        }).collect();

        sprite_renderer.queue(NamedTexture::SpriteSheet1, sprites.as_slice());
    }
}
