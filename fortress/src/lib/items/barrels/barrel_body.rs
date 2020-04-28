use crate::{
    entities::{
        Entity,
        RegisteredBody,
        RegisteredBodyBuilder,
    },
    items::barrels::{
        BarrelConfig,
        BarrelId,
    },
    physics::{
        collision_category,
        PhysicsSimulation,
    },
};
use nalgebra::Point2;
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

pub struct BarrelBody {
    body: RegisteredBody,
}

impl BarrelBody {
    pub fn new(config: &BarrelConfig, barrel_id: BarrelId, location: Point2<f64>, physics_sim: &mut PhysicsSimulation) -> Self {
        let rigid_body = RigidBodyDesc::new()
            .status(BodyStatus::Static)
            .translation(location.coords)
            .kinematic_rotations(false)
            .build();
        let ball_shape = Ball::new(config.physical_radius);
        let collider_desc = ColliderDesc::new(ShapeHandle::new(ball_shape))
            .density(config.physical_density)
            .sensor(true)
            .collision_groups(CollisionGroups::new()
                .with_membership(&[collision_category::BARREL])
                .with_whitelist(&[collision_category::BARRIER, collision_category::PLAYER_WEAPON]));

        let body = RegisteredBodyBuilder::new()
            .rigid_body(rigid_body)
            .collider(collider_desc)
            .entity(Entity::Barrel(barrel_id))
            .build(physics_sim);

        BarrelBody {
            body
        }
    }

    pub fn position(&self) -> Option<Point2<f64>> {
        self.body.default_position()
    }
}
