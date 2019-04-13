use crate::{
    entities::{
        Entity,
        RegisteredBody,
    },
    physics::{
        collision_category,
        PhysicsSimulation,
    },
    players::PlayerConfig,
};
use nalgebra::{
    Isometry2,
    Point2,
    Translation2,
    UnitComplex,
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
    algebra::{
        Force2,
        ForceType
    },
    object::{
        Body,
        BodyStatus,
        ColliderDesc,
        RigidBodyDesc,
    }
};

pub struct PlayerBody {
    pub body: RegisteredBody,
}

impl PlayerBody {
    pub fn new(config: &PlayerConfig, spawn: Point2<f64>, physics_sim: &mut PhysicsSimulation) -> PlayerBody {
        let ball_shape = Ball::new(config.physical_radius);
        let collider_desc = ColliderDesc::new(ShapeHandle::new(ball_shape))
            .translation(spawn.coords)
            .collision_groups(CollisionGroups::new()
                .with_membership(&[collision_category::PLAYER_BODY])
                .with_whitelist(&[collision_category::BARRIER, collision_category::PICKUP]));

        let mut rigid_body_desc = RigidBodyDesc::new()
            .status(BodyStatus::Dynamic)
            .collider(&collider_desc)
            .kinematic_rotation(true);
        let body_handle  = rigid_body_desc
            .build(physics_sim.borrow_mut().world_mut())
            .handle();

        PlayerBody {
            body: RegisteredBody::new(body_handle, Entity::Player, physics_sim),
        }
    }

    pub fn walk(&mut self, speed: f64, dir: Vector2<f64>) {
        let dir_magnitude = dir.norm();
        if !dir_magnitude.is_finite() {
            return;
        }

        let desired_velocity = (speed / dir_magnitude) * dir;

        let mut physics_sim = self.body.physics_sim.borrow_mut();
        if let Some(body) =  physics_sim.world_mut().rigid_body_mut(self.body.handle) {
            let actual_body_velocity = body.velocity().linear;
            let mass = body.augmented_mass().linear;
            let impulse = Force2::linear(mass * (desired_velocity - actual_body_velocity));
            body.apply_force(0 /*ignored part_id*/, &impulse, ForceType::Impulse, true /*one-time impulse*/);
        }
    }

    pub fn teleport_to(&mut self, point: Point2<f64>) {
        let mut physics_sim = self.body.physics_sim.borrow_mut();
        if let Some(body) =  physics_sim.world_mut().rigid_body_mut(self.body.handle) {
            let translation = Translation2::from(point.coords);
            body.set_position(Isometry2::from_parts(translation, UnitComplex::identity()));
        }
    }
}
