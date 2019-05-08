use crate::{
    items::{
        ItemConfig,
        ItemId,
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

pub struct ItemBody {
    body: RegisteredBody,
}

impl ItemBody {
    pub fn new(config: &ItemConfig, item_id: ItemId, spawn: Point2<f64>, physics_sim: &mut PhysicsSimulation) -> ItemBody {
        let ball_shape = Ball::new(config.item_physical_radius);
        let collider_desc = ColliderDesc::new(ShapeHandle::new(ball_shape))
            .density(config.item_physical_density)
            .sensor(true)
            .collision_groups(CollisionGroups::new()
                .with_membership(&[collision_category::ITEM])
                .with_whitelist(&[collision_category::BARRIER, collision_category::PLAYER_BODY]));

        let mut rigid_body_desc = RigidBodyDesc::new()
            .status(BodyStatus::Static)
            .translation(spawn.coords)
            .collider(&collider_desc)
            .kinematic_rotation(true);
        let body_handle  = rigid_body_desc
            .build(physics_sim.borrow_mut().world_mut())
            .handle();

        ItemBody {
            body: RegisteredBody::new(body_handle, Entity::Item(item_id), physics_sim)
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
