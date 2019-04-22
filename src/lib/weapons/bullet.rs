use crate::{
    dimensions::{
        Attack,
        Damage,
        time::{
            DeltaTime,
            Microseconds,
        },
    },
    entities::{
        Entity,
        RegisteredBody,
    },
    physics::{
        collision_category,
        PhysicsSimulation
    },
    players::PlayerConfig,
};
use nalgebra::{
    Point2,
    Vector2,
};
use ncollide2d::{
    shape::{
        Ball,
        ShapeHandle
    },
    world::CollisionGroups
};
use nphysics2d::{
    algebra::Velocity2,
    object::{
        BodyStatus,
        ColliderDesc,
        RigidBodyDesc,
    }
};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct BulletId(usize);

impl BulletId {
    pub fn new(val: usize) -> BulletId {
        BulletId(val)
    }

    pub fn to_usize(self) -> usize {
       self.0
   }
}

pub struct Bullet {
    body: RegisteredBody,
    time_elapsed: Microseconds,
}

impl Bullet {
    pub fn new(entity: Entity, radius: f64, start_position: Point2<f64>, velocity: Velocity2<f64>, physics_sim: &mut PhysicsSimulation) -> Bullet {
        let ball_shape = Ball::new(radius);
        let collider_desc = ColliderDesc::new(ShapeHandle::new(ball_shape))
            .density(radius)
            .collision_groups(CollisionGroups::new()
                .with_membership(&[collision_category::PLAYER_WEAPON])
                .with_whitelist(&[collision_category::BARRIER]));

        let mut rigid_body_desc = RigidBodyDesc::new()
            .status(BodyStatus::Dynamic)
            .translation(start_position.coords)
            .velocity(velocity)
            .collider(&collider_desc)
            .kinematic_rotation(true);
        let body_handle  = rigid_body_desc
            .build(physics_sim.borrow_mut().world_mut())
            .handle();

        Bullet {
            body: RegisteredBody::new(body_handle, entity, physics_sim),
            time_elapsed: 0,
        }
    }

    pub fn pre_update(&mut self, dt: DeltaTime) {
        self.time_elapsed += dt.as_microseconds();
    }

    pub fn frame(&self, config: &PlayerConfig) -> usize {
        (self.time_elapsed / config.bullet_sprite_frame_duration_micros) as usize
    }

    pub fn get_position(&self) -> Point2<f64> {
        let physics_sim = self.body.physics_sim.borrow();
        physics_sim
            .world()
            .rigid_body(self.body.handle)
            .map(|body| {
                Point2::from(body.position().translation.vector)
            })
            .unwrap_or(Point2::new(0.0, 0.0))
    }

    pub fn get_attack(&self, damage: Damage, knockback_strength: f64) -> Option<Attack> {
        let mut physics_sim = self.body.physics_sim.borrow_mut();
        physics_sim
            .world_mut()
            .rigid_body_mut(self.body.handle)
            .map(|body| {
                let velocity = body.velocity().linear;
                let velocity_mag = velocity.norm();
                let knockback_dir = if velocity_mag.is_normal() {
                    velocity / velocity_mag
                } else {
                    Vector2::new(0.0, 0.0)
                };

                Attack {
                    damage,
                    knockback_strength,
                    knockback_dir,
                }
            })
    }
}
