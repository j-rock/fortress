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
        PointLight,
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
    bullets: Slab<Bullet>,
    bullets_to_remove: Vec<BulletId>,
    current_delay: Option<time::Microseconds>,
    physics_sim: PhysicsSimulation,

    bullet_radius: f64,
    bullet_render_height: f32,
    bullet_light_color: glm::Vec3,
    bullet_light_attenuation: glm::Vec3,
}

impl Weapon {
    pub fn new(config: &PlayerConfig, physics_sim: &PhysicsSimulation) -> Weapon {
        Weapon {
            stats: WeaponStats::new(config),
            bullets: Slab::new(),
            bullets_to_remove: vec!(),
            current_delay: None,
            physics_sim: physics_sim.clone(),
            bullet_radius: config.bullet_radius,
            bullet_render_height: config.bullet_render_height,
            bullet_light_color: glm::vec3(config.bullet_light_color.0, config.bullet_light_color.1, config.bullet_light_color.2),
            bullet_light_attenuation: glm::vec3(config.bullet_light_attenuation.0, config.bullet_light_attenuation.1, config.bullet_light_attenuation.2),
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

    pub fn post_update(&mut self) {
        for bullet_id in self.bullets_to_remove.iter() {
            if self.bullets.contains(bullet_id.to_usize()) {
                self.bullets.remove(bullet_id.to_usize());
            }
        }
        self.bullets_to_remove.clear();
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

    pub fn bullet_hit(&mut self, bullet_id: BulletId) {
        self.bullets_to_remove.push(bullet_id);
    }

    pub fn populate_lights(&self, lights: &mut Vec<PointLight>) {
        for (_idx, bullet) in self.bullets.iter() {
            let body_position = bullet.get_position();
            let world_position = glm::vec3(body_position.x as f32, self.bullet_render_height, -body_position.y as f32);
            lights.push(PointLight {
                position: glm::vec3(world_position.x, world_position.y + self.bullet_radius as f32, world_position.z + 0.0001),
                color: self.bullet_light_color,
                attenuation: self.bullet_light_attenuation
            });
        }
    }

    pub fn queue_draw(&self, config: &PlayerConfig, sprite_renderer: &mut SpriteRenderer) {
        let sprites: Vec<_> = self.bullets.iter().map(|(_idx, bullet)| -> SpriteData {
            let body_position = bullet.get_position();
            let world_position = glm::vec3(body_position.x as f32, self.bullet_render_height, -body_position.y as f32);

            SpriteData {
                world_bottom_center_position: world_position,
                world_half_size: glm::vec2(self.bullet_radius as f32, self.bullet_radius as f32),
                tex_bottom_left: glm::vec2(config.bullet_texel_bottom_left.0, config.bullet_texel_bottom_left.1),
                tex_top_right: glm::vec2(config.bullet_texel_top_right.0, config.bullet_texel_top_right.1),
            }
        }).collect();

        sprite_renderer.queue(NamedTexture::SpriteSheet1, sprites.as_slice());
    }
}
