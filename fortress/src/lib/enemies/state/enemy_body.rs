use crate::{
    enemies::{
        EnemyConfig,
        EnemyId,
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

pub struct EnemyBody {
    body: RegisteredBody,
}

impl EnemyBody {
    pub fn new(config: &EnemyConfig, enemy_id: EnemyId, spawn: Point2<f64>, physics_sim: &mut PhysicsSimulation) -> EnemyBody {
        let rigid_body = RigidBodyDesc::new()
            .status(BodyStatus::Dynamic)
            .translation(spawn.coords)
            .kinematic_rotations(true)
            .build();

        let ball_shape = Ball::new(config.enemy_physical_radius);
        let collider_desc = ColliderDesc::new(ShapeHandle::new(ball_shape))
            .density(config.enemy_physical_density)
            .collision_groups(CollisionGroups::new()
                .with_membership(&[collision_category::ENEMY_BODY])
                .with_whitelist(&[collision_category::BARRIER, collision_category::ENEMY_BODY, collision_category::PLAYER_WEAPON]));

        let body = RegisteredBodyBuilder::new()
            .rigid_body(rigid_body)
            .collider(collider_desc)
            .entity(Entity::Enemy(enemy_id))
            .build(physics_sim);

        EnemyBody {
            body
        }
    }

    pub fn position(&self) -> Option<Point2<f64>> {
        self.body.default_position()
    }

    pub fn velocity(&self) -> Option<Vector2<f64>> {
        self.body.default_velocity()
    }

    pub fn move_to_target(&mut self, config: &EnemyConfig, player_locs: &Vec<Point2<f64>>) {
        if let Some(position) = self.position() {
            player_locs
                .iter()
                .min_by_key(|player_loc| {
                    let diff = position - **player_loc;
                    (diff.x * diff.x + diff.y * diff.y).round() as i64
                })
                .and_then(|closest_player_loc| -> Option<()> {
                    let displacement = *closest_player_loc - position;
                    let distance = displacement.norm();
                    if distance > config.enemy_stop_and_hit_distance && distance < config.enemy_anger_distance {
                        let desired_velocity = config.enemy_move_speed * displacement / distance;
                        self.body.default_set_velocity(desired_velocity);
                    }

                    None
                });
        }
    }
}
