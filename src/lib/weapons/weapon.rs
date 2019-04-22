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
    render::PointLight,
    weapons::{
        BulletId,
        Bullet,
        WeaponStats,
    },
};
use nalgebra::{
    Point2,
    Vector2
};
use nphysics2d::algebra::Velocity2;
use slab::Slab;
use crate::render::{FullyIlluminatedSpriteRenderer, FullyIlluminatedSpriteData};

pub struct Weapon {
    stats: WeaponStats,
    bullets: Slab<Bullet>,
    bullets_to_remove: Vec<BulletId>,
    current_delay: Option<time::Microseconds>,
    physics_sim: PhysicsSimulation,

    bullet_radius: f64,
}

impl Weapon {
    pub fn new(config: &PlayerConfig, physics_sim: &PhysicsSimulation) -> Weapon {
        Weapon {
            stats: WeaponStats::new(config),
            bullets: Slab::new(),
            bullets_to_remove: vec!(),
            current_delay: None,
            physics_sim: physics_sim.clone(),
            bullet_radius: config.bullet_physical_radius,
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

        for (_idx, bullet) in self.bullets.iter_mut() {
            bullet.pre_update(dt);
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

    pub fn populate_lights(&self, config: &PlayerConfig, lights: &mut Vec<PointLight>) {
        for (_idx, bullet) in self.bullets.iter() {
            lights.push(bullet.point_light(config));
        }
    }

    pub fn queue_draw(&self, config: &PlayerConfig, full_light: &mut FullyIlluminatedSpriteRenderer) {
        let sprites: Vec<_> = self.bullets.iter().map(|(_idx, bullet)| -> FullyIlluminatedSpriteData {
            bullet.render_info(config)
        }).collect();

        full_light.queue(sprites);
    }
}
