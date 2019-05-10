use crate::{
    dimensions::{
        Attack,
        Damage,
        Reverse,
        time::{
            DeltaTime,
            Microseconds,
        },
    },
    entities::{
        Entity,
        RegisteredBody,
    },
    physics::{
        collision_category,
        PhysicsSimulation
    },
    players::PlayerConfig,
    render::{
        NamedSpriteSheet,
        FullyIlluminatedSpriteData,
        PointLight,
        SpriteSheetFrameId,
    },
};
use generational_slab::Key;
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
use nphysics2d::{
    algebra::Velocity2,
    object::{
        BodyStatus,
        ColliderDesc,
        RigidBodyDesc,
    }
};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct BulletId(Key);

impl BulletId {
    pub fn new(key: Key) -> BulletId {
        BulletId(key)
    }

    pub fn to_key(self) -> Key {
       self.0
   }
}

pub struct Bullet {
    body: RegisteredBody,
    time_elapsed: Microseconds,
}

impl Bullet {
    pub fn new(entity: Entity, radius: f64, start_position: Point2<f64>, velocity: Velocity2<f64>, physics_sim: &mut PhysicsSimulation) -> Bullet {
        let ball_shape = Ball::new(radius);
        let collider_desc = ColliderDesc::new(ShapeHandle::new(ball_shape))
            .density(radius)
            .collision_groups(CollisionGroups::new()
                .with_membership(&[collision_category::PLAYER_WEAPON])
                .with_whitelist(&[collision_category::ENEMY_BODY, collision_category::ENEMY_GENERATOR]));

        let mut rigid_body_desc = RigidBodyDesc::new()
            .status(BodyStatus::Dynamic)
            .translation(start_position.coords)
            .velocity(velocity)
            .collider(&collider_desc)
            .kinematic_rotation(true);
        let body_handle  = rigid_body_desc
            .build(physics_sim.borrow_mut().world_mut())
            .handle();

        Bullet {
            body: RegisteredBody::new(body_handle, entity, physics_sim),
            time_elapsed: 0,
        }
    }

    pub fn pre_update(&mut self, dt: DeltaTime) {
        self.time_elapsed += dt.as_microseconds();
    }

    pub fn expired(&self, config: &PlayerConfig) -> bool {
        self.time_elapsed >= config.bullet_lifetime_duration_micros
    }

    pub fn get_attack(&self, damage: Damage, knockback_strength: f64) -> Option<Attack> {
        let physics_sim = self.body.physics_sim.borrow();
        physics_sim
            .world()
            .rigid_body(self.body.handle)
            .map(|body| {
                let velocity = body.velocity().linear;
                let velocity_mag = velocity.norm();
                let knockback_dir = if velocity_mag.is_normal() {
                    velocity / velocity_mag
                } else {
                    Vector2::new(0.0, 0.0)
                };

                Attack {
                    damage,
                    knockback_strength,
                    knockback_dir,
                }
            })
    }

    pub fn render_info(&self, config: &PlayerConfig) -> FullyIlluminatedSpriteData {
        let world_position = self.get_render_world_position(config);
        let frame = (self.time_elapsed / config.bullet_sprite_frame_duration_micros) as usize;
        let direction = self.get_unit_direction();

        FullyIlluminatedSpriteData {
            world_center_position: world_position,
            world_half_size: glm::vec2(config.bullet_render_width, config.bullet_render_height),
            sprite_frame_id: SpriteSheetFrameId {
                name: String::from("shooting_fireball.png"),
                sprite_sheet: NamedSpriteSheet::SpriteSheet1
            },
            frame,
            rotation: Self::choose_rotation(direction),
            reverse: Reverse::none(),
        }
    }

    pub fn point_light(&self, config: &PlayerConfig) -> PointLight {
        let world_position = self.get_render_world_position(config);
        let direction = self.get_unit_direction();
        let light_position_x = world_position.x + (direction.x as f32) * config.bullet_render_width * 0.75;
        let light_position_z = world_position.z - (direction.y as f32) * config.bullet_render_width * 0.75;

        PointLight {
            position: glm::vec3(light_position_x, world_position.y, light_position_z),
            color: glm::vec3(config.bullet_light_color.0, config.bullet_light_color.1, config.bullet_light_color.2),
            attenuation: glm::vec3(config.bullet_light_attenuation.0, config.bullet_light_attenuation.1, config.bullet_light_attenuation.2),
        }
    }

    fn get_render_world_position(&self, config: &PlayerConfig) -> glm::Vec3 {
        let physics_sim = self.body.physics_sim.borrow();
        physics_sim
            .world()
            .rigid_body(self.body.handle)
            .map(|body| {
                let body_position = body.position().translation.vector;
                glm::vec3(body_position.x as f32, config.bullet_render_elevation, -body_position.y as f32)
            })
            .unwrap_or(glm::vec3(0.0, config.bullet_render_elevation, 0.0))
    }

    fn get_unit_direction(&self) -> Vector2<f64> {
        let physics_sim = self.body.physics_sim.borrow();
        physics_sim
            .world()
            .rigid_body(self.body.handle)
            .and_then(|body| {
                let velocity = body.velocity().linear;
                let speed = velocity.norm();
                if !speed.is_normal() {
                    return None;
                }
                Some(velocity / speed)
            })
            .unwrap_or(Vector2::new(0.0, 0.0))
    }

    fn choose_rotation(direction: Vector2<f64>) -> f32 {
        let direction = Vector2::new(-direction.x as f32, direction.y as f32);

        180.0 / std::f32::consts::PI * match (direction.x.is_sign_positive(), direction.y.is_sign_positive()) {
            // Quadrant I + Quadrant IV
            (true, _) => direction.y.asin(),
            // Quadrant II
            (false, true) => std::f32::consts::PI / 2.0 + (-direction.x).asin(),
            // Quadrant III
            (false, false) => (-direction.x).acos() - std::f32::consts::PI,
        }
    }
}
