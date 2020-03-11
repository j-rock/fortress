use crate::{
    app::RandGen,
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
        PointLights,
    },
    weapons::{
        Bullet,
        BulletAttackType,
        BulletElement,
        BulletId,
        BulletTraits,
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
    bullet_element: BulletElement,
}

struct FireBulletArgs<'a> {
    stats: &'a PlayerStats,
    player_id: PlayerId,
    start_position: Point2<f64>,
    direction: Vector2<f64>,
    bullet_traits: BulletTraits,
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
            bullet_element: BulletElement::Fire,
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

    pub fn try_fire_normal(&mut self, audio: &AudioPlayer, stats: &PlayerStats, player_id: PlayerId, start_position: Point2<f64>, direction: Vector2<f64>, rng: &mut RandGen) {
        if self.current_normal_delay.is_none() {
            let args = FireBulletArgs {
                stats,
                player_id,
                start_position,
                direction,
                bullet_traits: BulletTraits::new(BulletAttackType::Regular, self.bullet_element),
            };
            if self.fire_one(args, rng) {
                self.current_normal_delay = Some(0);
                audio.play_sound(Sound::Blast);
            }
        }
    }

    pub fn try_fire_special(&mut self, config: &PlayerConfig, audio: &AudioPlayer, stats: &PlayerStats, player_id: PlayerId, start_position: Point2<f64>, direction: Vector2<f64>, rng: &mut RandGen) {
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
                let args = FireBulletArgs {
                    stats,
                    player_id,
                    start_position,
                    direction,
                    bullet_traits: BulletTraits::new(BulletAttackType::Special, self.bullet_element),
                };
                self.fire_one(args, rng)
            }) {
                self.current_special_delay = Some(0);
                audio.play_sound(Sound::Blast);
            }
        }
    }

    pub fn switch_bullet_element(&mut self) {
        self.bullet_element = match self.bullet_element {
            BulletElement::Fire => BulletElement::Poison,
            BulletElement::Poison => BulletElement::Ice,
            BulletElement::Ice => BulletElement::Fire,
        }
    }

    pub fn bullet_hit(&mut self, bullet_id: BulletId) {
        if let Some(bullet) = self.bullets.get(bullet_id.to_key()) {
           if !bullet.remove_on_collision() {
               return;
           }
        }

        self.bullets_to_remove.push(bullet_id);
    }

    pub fn bullet_attack(&self, stats: &PlayerStats, bullet_id: BulletId) -> Option<Attack> {
        self.bullets
            .get(bullet_id.to_key())
            .and_then(|bullet| {
                bullet.get_attack(stats.get_bullet_damage(), stats.get_knockback_strength())
            })
    }

    pub fn populate_lights(&self, config: &PlayerConfig, lights: &mut PointLights) {
        let queue_data = self.bullets
            .iter()
            .map(|(_idx, bullet)| {
                bullet.point_light(config)
            });
        lights.append(queue_data);
    }

    pub fn queue_draw(&self, config: &PlayerConfig, full_light: &mut FullyIlluminatedSpriteRenderer) {
        let sprites = self.bullets.iter().map(|(_idx, bullet)| -> FullyIlluminatedSpriteData {
            bullet.render_info(config)
        });

        full_light.queue(sprites);
    }

    fn fire_one(&mut self, args: FireBulletArgs, rng: &mut RandGen) -> bool {
        let vacant_entry = self.bullets.vacant_entry();
        let bullet_id = BulletId::new(vacant_entry.key());
        let entity = Entity::Bullet(args.player_id, bullet_id);

        let bullet_speed = args.stats.get_bullet_speed();
        let linear_vel = bullet_speed * args.direction;
        let velocity = Velocity2::linear(linear_vel.x, linear_vel.y);

        let bullet = Bullet::new(entity, args.bullet_traits, self.bullet_radius, args.start_position, velocity, rng, &mut self.physics_sim);
        vacant_entry.insert(bullet)
    }
}
