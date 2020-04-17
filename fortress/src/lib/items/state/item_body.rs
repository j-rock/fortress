use crate::{
    items::{
        ItemConfig,
        ItemId,
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

pub struct ItemBody {
    body: RegisteredBody,
}

impl ItemBody {
    pub fn new(config: &ItemConfig, item_id: ItemId, spawn: Point2<f64>, physics_sim: &mut PhysicsSimulation) -> ItemBody {
        let rigid_body = RigidBodyDesc::new()
            .status(BodyStatus::Static)
            .translation(spawn.coords)
            .kinematic_rotations(true)
            .build();
        let ball_shape = Ball::new(config.physical_radius);
        let collider_desc = ColliderDesc::new(ShapeHandle::new(ball_shape))
            .density(config.physical_density)
            .sensor(true)
            .collision_groups(CollisionGroups::new()
                .with_membership(&[collision_category::ITEM])
                .with_whitelist(&[collision_category::BARRIER, collision_category::PLAYER_BODY]));

        let body = RegisteredBodyBuilder::new()
            .rigid_body(rigid_body)
            .collider(collider_desc)
            .entity(Entity::Item(item_id))
            .build(physics_sim);

        ItemBody {
            body
        }
    }

    pub fn position(&self) -> Option<Point2<f64>> {
        self.body.default_position()
    }
}
