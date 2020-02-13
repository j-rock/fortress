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
                .with_whitelist(&[collision_category::BARRIER, collision_category::ENEMY_BODY, collision_category::PLAYER_WEAPON]));

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

    pub fn velocity(&self) -> Option<Vector2<f64>> {
        let physics_sim = self.body.physics_sim.borrow();
        physics_sim
            .world()
            .rigid_body(self.body.handle)
            .map(|body| {
                body.velocity().linear
            })
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
                        self.set_velocity(desired_velocity);
                    }

                    None
                });
        }
    }

    fn set_velocity(&mut self, desired_velocity: Vector2<f64>) {
        let mut physics_sim = self.body.physics_sim.borrow_mut();
        if let Some(body) =  physics_sim.world_mut().rigid_body_mut(self.body.handle) {
            let actual_body_velocity = body.velocity().linear;
            let mass = body.augmented_mass().linear;
            let impulse = Force2::linear(mass * (desired_velocity - actual_body_velocity));
            body.apply_force(0 /*ignored part_id*/, &impulse, ForceType::Impulse, true /*one-time impulse*/);
        }
    }
}
