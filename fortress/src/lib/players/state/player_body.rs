use crate::{
    entities::{
        Entity,
        RegisteredBody,
        RegisteredBodyBuilder,
    },
    physics::{
        collision_category,
        PhysicsSimulation,
    },
    players::{
        PlayerConfig,
        PlayerId,
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

pub struct PlayerBody {
    pub body: RegisteredBody,
}

impl PlayerBody {
    pub fn new(config: &PlayerConfig, player_id: PlayerId, spawn: Point2<f64>, physics_sim: &mut PhysicsSimulation) -> PlayerBody {
        let rigid_body = RigidBodyDesc::new()
            .status(BodyStatus::Dynamic)
            .translation(spawn.coords)
            .kinematic_rotations(true)
            .build();
        let ball_shape = Ball::new(config.physical_radius);
        let collider_desc = ColliderDesc::new(ShapeHandle::new(ball_shape))
            .density(config.physical_density)
            .collision_groups(CollisionGroups::new()
                .with_membership(&[collision_category::PLAYER_BODY])
                .with_whitelist(&[collision_category::BARRIER, collision_category::ITEM]));

        let body = RegisteredBodyBuilder::new()
            .rigid_body(rigid_body)
            .collider(collider_desc)
            .entity(Entity::Player(player_id))
            .build(physics_sim);


        PlayerBody {
            body
        }
    }

    pub fn set_velocity(&mut self, desired_velocity: Vector2<f64>) {
        self.body.default_set_velocity(desired_velocity);
    }

    pub fn teleport_to(&mut self, point: Point2<f64>) {
        self.body.default_set_position(point);
    }

    pub fn position(&self) -> Option<Point2<f64>> {
        self.body.default_position()
    }
}
