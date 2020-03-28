use crate::{
    enemies::{
        EnemyGeneratorConfig,
        EnemyGeneratorId,
        EnemyGeneratorSpawn,
    },
    entities::{
        Entity,
        RegisteredBody,
        RegisteredBodyBuilder,
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
    pipeline::object::CollisionGroups,
    shape::{
        Ball,
        ShapeHandle
    },
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
    pub fn new(config: &EnemyGeneratorConfig, generator_id: EnemyGeneratorId, spawn: EnemyGeneratorSpawn, physics_sim: &mut PhysicsSimulation) -> EnemyGeneratorBody {
        let rigid_body = RigidBodyDesc::new()
            .status(BodyStatus::Dynamic)
            .translation(Vector2::new(spawn.position.0, spawn.position.1))
            .kinematic_rotations(true)
            .build();
        let ball_shape = Ball::new(config.physical_radius);
        let collider_desc = ColliderDesc::new(ShapeHandle::new(ball_shape))
            .density(config.physical_density)
            .sensor(true)
            .collision_groups(CollisionGroups::new()
                .with_membership(&[collision_category::ENEMY_GENERATOR])
                .with_whitelist(&[collision_category::BARRIER, collision_category::PLAYER_WEAPON]));

        let body = RegisteredBodyBuilder::new()
            .rigid_body(rigid_body)
            .collider(collider_desc)
            .entity(Entity::EnemyGenerator(generator_id))
            .build(physics_sim);

        EnemyGeneratorBody {
            body,
            orientation: spawn.orientation
        }
    }

    pub fn position(&self) -> Option<Point2<f64>> {
        self.body.default_position()
    }

    pub fn orientation(&self) -> f64 {
        self.orientation
    }
}
