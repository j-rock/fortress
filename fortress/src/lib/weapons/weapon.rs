use crate::{
    app::RandGen,
    dimensions::{
        Attack,
        time::{
            DeltaTime,
            Timer,
        }
    },
    entities::Entity,
    physics::PhysicsSimulation,
    players::{
        PlayerBulletConfig,
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
    current_normal_delay: Timer,
    current_special_delay: Timer,
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
    pub fn new(config: &PlayerBulletConfig, physics_sim: &PhysicsSimulation) -> Weapon {
        Weapon {
            bullets: Slab::new(),
            bullets_to_remove: vec!(),
            current_normal_delay: Timer::expired(),
            current_special_delay: Timer::expired(),
            physics_sim: physics_sim.clone(),
            bullet_radius: config.physical_radius,
            bullet_element: BulletElement::Poison,
        }
    }

    pub fn pre_update(&mut self, dt: DeltaTime) {
        self.current_normal_delay.tick(dt);
        self.current_special_delay.tick(dt);

        for (key, bullet) in self.bullets.iter_mut() {
            bullet.pre_update(dt);
            if bullet.expired() {
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

    pub fn try_fire_normal(&mut self,
                           config: &PlayerBulletConfig,
                           stats: &PlayerStats,
                           player_id: PlayerId,
                           start_position: Point2<f64>,
                           direction: Vector2<f64>,
                           rng: &mut RandGen) -> bool {
        if !self.current_normal_delay.is_expired() {
            return false;
        }
        let args = FireBulletArgs {
            stats,
            player_id,
            start_position,
            direction,
            bullet_traits: BulletTraits::new(BulletAttackType::Regular, self.bullet_element),
        };
        if self.fire_one(config, args, rng) {
            self.current_normal_delay = Timer::new(stats.get_normal_firing_period());
            true
        } else {
            false
        }
    }

    pub fn try_fire_special(&mut self,
                            config: &PlayerBulletConfig,
                            stats: &PlayerStats,
                            player_id: PlayerId,
                            start_position: Point2<f64>,
                            direction: Vector2<f64>,
                            rng: &mut RandGen) -> bool {
        if !self.current_special_delay.is_expired() {
            return false;
        }

        let rot_left = Rotation2::new(config.special_spread_radians);
        let rot_right = Rotation2::new(-config.special_spread_radians);
        let mut dir_left = direction;
        let mut dir_right = direction;
        let mut directions = Vec::with_capacity(2 * config.special_num_shots + 1);
        for _i in 0..config.special_num_shots {
            dir_left = rot_left * dir_left;
            dir_right = rot_right * dir_right;
            directions.push(dir_left);
            directions.push(dir_right);
        }
        directions.push(direction);

        let mut fired_any = false;
        for direction in directions.into_iter() {
            let args = FireBulletArgs {
                stats,
                player_id,
                start_position,
                direction,
                bullet_traits: BulletTraits::new(BulletAttackType::Special, self.bullet_element),
            };
            fired_any |= self.fire_one(config, args, rng);
        }
        if fired_any {
            self.current_special_delay = Timer::new(stats.get_special_firing_period());
        }
        return fired_any;
    }

    pub fn switch_bullet_element(&mut self) {
        self.bullet_element = match self.bullet_element {
            BulletElement::Fire => BulletElement::Poison,
            BulletElement::Poison => BulletElement::Ice,
            BulletElement::Ice => BulletElement::Fire,
        }
    }

    pub fn bullet_hit(&mut self, bullet_id: BulletId) -> Option<Vector2<f64>> {
        let bullet = self.bullets.get(bullet_id.to_key())?;
        if bullet.remove_on_collision() {
            self.bullets_to_remove.push(bullet_id);
        }
        bullet.direction()
    }

    pub fn bullet_attack(&self, stats: &PlayerStats, bullet_id: BulletId) -> Option<Attack> {
        self.bullets
            .get(bullet_id.to_key())
            .and_then(|bullet| {
                bullet.get_attack(stats.get_bullet_damage(), stats.get_knockback_strength())
            })
    }

    pub fn populate_lights(&self, config: &PlayerBulletConfig, lights: &mut PointLights) {
        let queue_data = self.bullets
            .iter()
            .map(|(_idx, bullet)| {
                bullet.point_light(config)
            });
        lights.append(queue_data);
    }

    pub fn queue_draw(&self, config: &PlayerBulletConfig, full_light: &mut FullyIlluminatedSpriteRenderer) {
        let sprites = self.bullets.iter().map(|(_idx, bullet)| -> FullyIlluminatedSpriteData {
            bullet.render_info(config)
        });

        full_light.queue(sprites);
    }

    fn fire_one(&mut self, config: &PlayerBulletConfig, args: FireBulletArgs, rng: &mut RandGen) -> bool {
        let vacant_entry = self.bullets.vacant_entry();
        let bullet_id = BulletId::new(vacant_entry.key());
        let entity = Entity::Bullet(args.player_id, bullet_id);

        let bullet_speed = args.stats.get_bullet_speed();
        let linear_vel = bullet_speed * args.direction;
        let velocity = Velocity2::linear(linear_vel.x, linear_vel.y);

        let bullet =
            Bullet::new(config,
                        entity,
                        args.bullet_traits,
                        self.bullet_radius,
                        args.start_position,
                        velocity,
                        rng,
                        &mut self.physics_sim);
        vacant_entry.insert(bullet)
    }
}
