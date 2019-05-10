use crate::{
    audio::{
        AudioPlayer,
        Sound,
    },
    dimensions::{
        Attack,
        time::{
            DeltaTime,
            Microseconds,
        }
    },
    entities::Entity,
    physics::PhysicsSimulation,
    players::{
        PlayerConfig,
        PlayerId,
        state::PlayerStats,
    },
    render::{
        FullyIlluminatedSpriteRenderer,
        FullyIlluminatedSpriteData,
        PointLight,
    },
    weapons::{
        BulletId,
        Bullet,
    },
};
use generational_slab::Slab;
use nalgebra::{
    Point2,
    Rotation2,
    Vector2,
};
use nphysics2d::algebra::Velocity2;

pub struct Weapon {
    bullets: Slab<Bullet>,
    bullets_to_remove: Vec<BulletId>,
    current_normal_delay: Option<Microseconds>,
    current_special_delay: Option<Microseconds>,
    physics_sim: PhysicsSimulation,

    bullet_radius: f64,
}

impl Weapon {
    pub fn new(config: &PlayerConfig, physics_sim: &PhysicsSimulation) -> Weapon {
        Weapon {
            bullets: Slab::new(),
            bullets_to_remove: vec!(),
            current_normal_delay: None,
            current_special_delay: None,
            physics_sim: physics_sim.clone(),
            bullet_radius: config.bullet_physical_radius,
        }
    }

    pub fn pre_update(&mut self, config: &PlayerConfig, stats: &PlayerStats, dt: DeltaTime) {
        let current_normal_delay = self.current_normal_delay.unwrap_or(stats.get_normal_firing_period()) + dt.as_microseconds();
        self.current_normal_delay = if current_normal_delay >= stats.get_normal_firing_period() {
            None
        } else {
            Some(current_normal_delay)
        };

        let current_special_delay = self.current_special_delay.unwrap_or(stats.get_special_firing_period()) + dt.as_microseconds();
        self.current_special_delay = if current_special_delay >= stats.get_special_firing_period() {
            None
        } else {
            Some(current_special_delay)
        };

        for (key, bullet) in self.bullets.iter_mut() {
            bullet.pre_update(dt);
            if bullet.expired(config) {
               self.bullets_to_remove.push(BulletId::new(key));
            }
        }
    }

    pub fn post_update(&mut self) {
        for bullet_id in self.bullets_to_remove.iter() {
            self.bullets.remove(bullet_id.to_key());
        }
        self.bullets_to_remove.clear();
    }

    pub fn try_fire_normal(&mut self, audio: &AudioPlayer, stats: &PlayerStats, player_id: PlayerId, start_position: Point2<f64>, direction: Vector2<f64>) {
        if self.current_normal_delay.is_none() && self.fire_one(stats, player_id, start_position, direction) {
            self.current_normal_delay = Some(0);
            audio.play_sound(Sound::Blast);
        }
    }

    pub fn try_fire_special(&mut self, config: &PlayerConfig, audio: &AudioPlayer, stats: &PlayerStats, player_id: PlayerId, start_position: Point2<f64>, direction: Vector2<f64>) {
        if self.current_special_delay.is_none() {
            let rot_left = Rotation2::new(config.bullet_special_spread_radians);
            let rot_right = Rotation2::new(-config.bullet_special_spread_radians);
            let mut dir_left = direction;
            let mut dir_right = direction;
            let mut directions = Vec::with_capacity(2 * config.bullet_special_num_shots + 1);
            for _i in 0..config.bullet_special_num_shots {
                dir_left = rot_left * dir_left;
                dir_right = rot_right * dir_right;
                directions.push(dir_left);
                directions.push(dir_right);
            }
            directions.push(direction);

            if directions.into_iter().all(|direction| {
                self.fire_one(stats, player_id, start_position, direction)
            }) {
                self.current_special_delay = Some(0);
                audio.play_sound(Sound::Blast);
            }
        }
    }

    pub fn bullet_hit(&mut self, bullet_id: BulletId) {
        self.bullets_to_remove.push(bullet_id);
    }

    pub fn bullet_attack(&self, stats: &PlayerStats, bullet_id: BulletId) -> Option<Attack> {
        self.bullets
            .get(bullet_id.to_key())
            .and_then(|bullet| {
                bullet.get_attack(stats.get_bullet_damage(), stats.get_knockback_strength())
            })
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

    fn fire_one(&mut self, stats: &PlayerStats, player_id: PlayerId, start_position: Point2<f64>, direction: Vector2<f64>) -> bool {
        let vacant_entry = self.bullets.vacant_entry();
        let bullet_id = BulletId::new(vacant_entry.key());
        let entity = Entity::Bullet(player_id, bullet_id);

        let bullet_speed = stats.get_bullet_speed();
        let linear_vel = bullet_speed * direction;
        let velocity = Velocity2::linear(linear_vel.x, linear_vel.y);

        let bullet = Bullet::new(entity, self.bullet_radius, start_position, velocity, &mut self.physics_sim);
        vacant_entry.insert(bullet)
    }
}
