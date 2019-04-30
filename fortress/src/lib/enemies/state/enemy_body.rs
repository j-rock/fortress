use crate::{
    enemies::{
        EnemyConfig,
        EnemyId,
    },
    entities::{
        Entity,
        RegisteredBody,
    },
    physics::{
        collision_category,
        PhysicsSimulation,
    },
};
use nalgebra::Point2;
use ncollide2d::{
    shape::{
        Ball,
        ShapeHandle
    },
    world::CollisionGroups
};
use nphysics2d::object::{
    BodyStatus,
    ColliderDesc,
    RigidBodyDesc,
};

pub struct EnemyBody {
    body: RegisteredBody,
}

impl EnemyBody {
    pub fn new(config: &EnemyConfig, enemy_id: EnemyId, spawn: Point2<f64>, physics_sim: &mut PhysicsSimulation) -> EnemyBody {
        let ball_shape = Ball::new(config.enemy_physical_radius);
        let collider_desc = ColliderDesc::new(ShapeHandle::new(ball_shape))
            .density(config.enemy_physical_density)
            .collision_groups(CollisionGroups::new()
                .with_membership(&[collision_category::ENEMY_BODY])
                .with_whitelist(&[collision_category::BARRIER, collision_category::PLAYER_WEAPON]));

        let mut rigid_body_desc = RigidBodyDesc::new()
            .status(BodyStatus::Dynamic)
            .translation(spawn.coords)
            .collider(&collider_desc)
            .kinematic_rotation(true);
        let body_handle  = rigid_body_desc
            .build(physics_sim.borrow_mut().world_mut())
            .handle();

        EnemyBody {
            body: RegisteredBody::new(body_handle, Entity::Enemy(enemy_id), physics_sim)
        }
    }

    pub fn position(&self) -> Option<Point2<f64>> {
        let physics_sim = self.body.physics_sim.borrow();
        physics_sim
            .world()
            .rigid_body(self.body.handle)
            .map(|body| {
                Point2::from(body.position().translation.vector)
            })
    }
}
