use crate::{
    dimensions::{
        LrDirection,
        OctoDirection,
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
    body: RegisteredBody,
    spawn: Point2<f64>,
    facing_dir: Vector2<f64>,
    lr_direction: LrDirection,
}

impl PlayerBody {
    pub fn new(config: &PlayerConfig,
               player_id: PlayerId,
               spawn: Point2<f64>,
               physics_sim: &mut PhysicsSimulation) -> PlayerBody {
        let rigid_body = RigidBodyDesc::new()
            .status(BodyStatus::Dynamic)
            .translation(spawn.clone().coords)
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
            body,
            spawn,
            facing_dir: Vector2::new(1.0, 0.0),
            lr_direction: LrDirection::Right,
        }
    }

    pub fn redeploy(&mut self, config: &PlayerConfig, player_id: PlayerId, physics_sim: &mut PhysicsSimulation) {
        let mut new_body = PlayerBody::new(config, player_id, self.spawn.clone(), physics_sim);
        new_body.facing_dir = self.facing_dir.clone();
        new_body.lr_direction = self.lr_direction;

        *self = new_body;
    }

    pub fn respawn(&mut self, spawn: Point2<f64>) {
        self.spawn = spawn;
        self.body.default_set_position(self.spawn.clone());
    }

    pub fn shove_backward(&mut self, magnitude: f64) {
        let backward = -self.facing_dir.clone();
        self.body.default_apply_impulse(backward, magnitude);
    }

    pub fn stop_moving(&mut self) {
        self.body.default_set_velocity(Vector2::new(0.0, 0.0));
    }

    pub fn move_forward(&mut self, speed: f64) {
        let desired_velocity = self.facing_dir.clone() * speed;
        self.body.default_set_velocity(desired_velocity);
    }

    pub fn position(&self) -> Option<Point2<f64>> {
        self.body.default_position()
    }

    pub fn facing_dir(&self) -> Vector2<f64> {
        self.facing_dir.clone()
    }

    pub fn lr_direction(&self) -> LrDirection {
        self.lr_direction
    }

    pub fn update_direction(&mut self, dir: OctoDirection) {
        self.facing_dir = dir.to_direction();
        if let Some(lr_direction) = dir.to_lr_direction() {
            self.lr_direction = lr_direction;
        }
    }
}
