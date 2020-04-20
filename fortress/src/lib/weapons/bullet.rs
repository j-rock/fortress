use crate::{
    app::RandGen,
    dimensions::{
        Attack,
        Damage,
        Reverse,
        time::{
            DeltaTime,
            Microseconds,
            Timer,
        },
    },
    entities::{
        Entity,
        RegisteredBody,
        RegisteredBodyBuilder,
    },
    physics::{
        collision_category,
        PhysicsSimulation
    },
    players::PlayerBulletConfig,
    render::{
        NamedSpriteSheet,
        FullyIlluminatedSpriteData,
        PointLight,
        SpriteSheetFrameId,
    },
    weapons::BulletTraits,
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
    algebra::Velocity2,
    object::{
        BodyStatus,
        ColliderDesc,
        RigidBodyDesc,
    }
};

pub struct Bullet {
    body: RegisteredBody,
    time_left: Timer,
    bullet_traits: BulletTraits,
    unit_random: f32,
}

impl Bullet {
    pub fn new(config: &PlayerBulletConfig,
               entity: Entity,
               bullet_traits: BulletTraits,
               radius: f64,
               start_position: Point2<f64>,
               velocity: Velocity2<f64>,
               rng: &mut RandGen,
               physics_sim: &mut PhysicsSimulation) -> Bullet {
        let rigid_body = RigidBodyDesc::new()
            .status(BodyStatus::Dynamic)
            .translation(start_position.coords)
            .velocity(velocity)
            .kinematic_rotations(true)
            .build();
        let ball_shape = Ball::new(radius);
        let collider_desc = ColliderDesc::new(ShapeHandle::new(ball_shape))
            .density(radius)
            .sensor(true)
            .collision_groups(CollisionGroups::new()
                .with_membership(&[collision_category::PLAYER_WEAPON])
                .with_whitelist(&[collision_category::ENEMY_BODY, collision_category::ENEMY_GENERATOR]));

        let body = RegisteredBodyBuilder::new()
            .rigid_body(rigid_body)
            .collider(collider_desc)
            .entity(entity)
            .build(physics_sim);

        Bullet {
            body,
            time_left: Timer::new(config.lifetime_duration_micros),
            unit_random: rng.unit_f32(),
            bullet_traits,
        }
    }

    pub fn pre_update(&mut self, dt: DeltaTime) {
        self.time_left.tick(dt);
    }

    pub fn expired(&self) -> bool {
        self.time_left.is_expired()
    }

    pub fn remove_on_collision(&self) -> bool {
        self.bullet_traits.remove_on_collision()
    }

    pub fn get_attack(&self, damage: Damage, knockback_strength: f64) -> Option<Attack> {
        let velocity = self.body.default_velocity()?;
        let velocity_mag = velocity.norm();
        let knockback_dir = if velocity_mag.is_normal() {
            velocity / velocity_mag
        } else {
            Vector2::new(0.0, 0.0)
        };

        Some(Attack {
            damage,
            knockback_strength,
            knockback_dir,
        })
    }

    pub fn render_info(&self, config: &PlayerBulletConfig) -> FullyIlluminatedSpriteData {
        let world_position = self.get_render_world_position(config);

        let rand_frame_offset = self.unit_random * (config.sprite_num_frames as f32 * config.sprite_frame_duration_micros as f32);
        let rand_frame_offset = rand_frame_offset as Microseconds;
        let frame_duration = config.lifetime_duration_micros - self.time_left.time_left() + rand_frame_offset;
        let frame = (frame_duration / config.sprite_frame_duration_micros) as usize;

        FullyIlluminatedSpriteData {
            world_center_position: world_position,
            world_half_size: glm::vec2(config.render_width, config.render_height),
            sprite_frame_id: SpriteSheetFrameId::new(String::from(self.bullet_traits.sprite_sheet_image_name()), NamedSpriteSheet::SpriteSheet1),
            frame,
            unit_world_rotation: self.get_unit_direction(),
            reverse: Reverse::none(),
            bloom_intensity: config.bloom_intensity,
        }
    }

    pub fn point_light(&self, config: &PlayerBulletConfig) -> PointLight {
        let world_position = self.get_render_world_position(config);
        let direction = self.get_unit_direction();
        let light_position_x = world_position.x + (direction.x as f32) * config.render_width * 0.75;
        let light_position_z = world_position.z - (direction.y as f32) * config.render_width * 0.75;
        let color = self.bullet_traits.light_color(config);

        let position = glm::vec3(light_position_x, world_position.y, light_position_z);
        let attenuation = glm::vec3(config.light_attenuation.0, config.light_attenuation.1, config.light_attenuation.2);
        PointLight::new(position, color, attenuation)
    }

    pub fn direction(&self) -> Option<Vector2<f64>> {
        self.body
            .default_velocity()
            .map(|velocity| {
                velocity.normalize()
            })
    }

    fn get_render_world_position(&self, config: &PlayerBulletConfig) -> glm::Vec3 {
        self.body.default_position()
            .map(|body_position| {
                glm::vec3(body_position.x as f32, config.render_elevation, -body_position.y as f32)
            })
            .unwrap_or(glm::vec3(0.0, config.render_elevation, 0.0))
    }

    fn get_unit_direction(&self) -> Vector2<f64> {
        self.body.default_velocity()
            .and_then(|velocity| {
                let speed = velocity.norm();
                if !speed.is_normal() {
                    return None;
                }
                Some(velocity / speed)
            })
            .unwrap_or(Vector2::new(0.0, 0.0))
    }
}
