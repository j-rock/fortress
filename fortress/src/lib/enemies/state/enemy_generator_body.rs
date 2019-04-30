use crate::{
    enemies::{
        EnemyConfig,
        EnemyGeneratorId,
        EnemyGeneratorSpawn,
    },
    entities::{
        Entity,
        RegisteredBody
    },
    physics::{
        collision_category,
        PhysicsSimulation,
    },
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
use nphysics2d::object::{
    BodyStatus,
    ColliderDesc,
    RigidBodyDesc,
};

pub struct EnemyGeneratorBody {
    body: RegisteredBody,
    orientation: f64,
}

impl EnemyGeneratorBody {
    pub fn new(config: &EnemyConfig, generator_id: EnemyGeneratorId, spawn: EnemyGeneratorSpawn, physics_sim: &mut PhysicsSimulation) -> EnemyGeneratorBody {
        let ball_shape = Ball::new(config.generator_physical_radius);
        let collider_desc = ColliderDesc::new(ShapeHandle::new(ball_shape))
            .density(config.generator_physical_density)
            .collision_groups(CollisionGroups::new()
                .with_membership(&[collision_category::ENEMY_GENERATOR])
                .with_whitelist(&[collision_category::BARRIER, collision_category::PLAYER_WEAPON]));

        let mut rigid_body_desc = RigidBodyDesc::new()
            .status(BodyStatus::Dynamic)
            .translation(Vector2::new(spawn.position.0, spawn.position.1))
            .collider(&collider_desc)
            .kinematic_rotation(true);
        let body_handle = rigid_body_desc
            .build(physics_sim.borrow_mut().world_mut())
            .handle();

        EnemyGeneratorBody {
            body: RegisteredBody::new(body_handle, Entity::EnemyGenerator(generator_id), physics_sim),
            orientation: spawn.orientation
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

    pub fn orientation(&self) -> f64 {
        self.orientation
    }
}
