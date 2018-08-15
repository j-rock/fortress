use dimensions::LrDirection;
use liquidfun::box2d::{
    collision::shapes::polygon_shape::PolygonShape,
    common::math::Vec2,
    dynamics::{
        body::{
            Body,
            BodyDef,
            BodyType,
        },
        fixture::FixtureDef,
        world::World,
    },
};
use physics::collision_category;
use wraith::WraithConfig;

pub struct WraithBody {
    pub body: Body,
    facing_dir: LrDirection,
}

impl WraithBody {
    pub fn new(config: &WraithConfig, world: &mut World) -> WraithBody {
        let mut body_def = BodyDef::default();
        body_def.body_type = BodyType::DynamicBody;
        body_def.position = Vec2::new(config.spawn_location.0 as f32, config.spawn_location.1 as f32);
        body_def.fixed_rotation = true;

        let body = world.create_body(&body_def);

        // Wraith body fixture
        let mut poly_shape = PolygonShape::new();
        {
            let (hx, hy) = (config.size.0 as f32 / 2.0, config.size.1 as f32 / 2.0);
            poly_shape.set_as_box(hx, hy);

            let mut fixture_def = FixtureDef::new(&poly_shape);
            fixture_def.density = config.density;
            fixture_def.filter.category_bits = collision_category::COLLIDE_ALL;
            fixture_def.filter.mask_bits = collision_category::MASK_ALLOW_ALL;
            body.create_fixture(&fixture_def);
        }

        WraithBody {
            body,
            facing_dir: LrDirection::Left,
        }
    }

    pub fn move_horizontal(&mut self, speed: f32, dir: Option<LrDirection>) {
        if let Some(dir) = dir {
            self.turn_face(dir);
        }

        let desired_horizontal_velocity = speed * match dir {
            None => 0.0,
            Some(LrDirection::Left) => -1.0,
            Some(LrDirection::Right) => 1.0
        };

        let actual_body_velocity = *self.body.get_linear_velocity();
        let mass = self.body.get_mass();
        let impulse = Vec2::new(mass * (desired_horizontal_velocity - actual_body_velocity.x), 0.0);
        let body_center = *self.body.get_world_center();
        self.body.apply_linear_impulse(&impulse, &body_center, true);
    }

    fn turn_face(&mut self, direction: LrDirection) {
        self.facing_dir = direction;
    }
}

impl Drop for WraithBody {
    fn drop(&mut self) {
        let mut world = self.body.get_world();
        world.destroy_body(&mut self.body);
    }
}

