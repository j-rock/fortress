use dimensions::LrDirection;
use entities::{
    Entity,
    EntityType,
    EntityRegistrar,
    RegisteredBody,
};
use liquidfun::box2d::{
    collision::shapes::polygon_shape::PolygonShape,
    common::math::Vec2,
    dynamics::{
        body::{
            BodyDef,
            BodyType,
        },
        fixture::FixtureDef,
        world::World,
    },
};
use physics::collision_category;
use wraiths::{
    Wraith,
    WraithConfig
};

pub struct WraithBody {
    pub body: RegisteredBody,
    facing_dir: LrDirection,
}

impl WraithBody {
    pub fn new(config: &WraithConfig, registrar: EntityRegistrar, world: &mut World) -> WraithBody {
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
            fixture_def.friction = config.friction;
            fixture_def.filter.category_bits = collision_category::WRAITH;
            fixture_def.filter.mask_bits = collision_category::BARRIER | collision_category::PLAYER_WEAPON;
            body.create_fixture(&fixture_def);
        }

        let body = RegisteredBody::new(body, registrar, None);

        WraithBody {
            body,
            facing_dir: LrDirection::Left,
        }
    }

    pub fn register(&mut self, wraith: *const Wraith) {
        let wraith_entity = Entity::new(EntityType::Wraith, wraith);
        self.body.register(wraith_entity);
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

        let body = &mut self.body.data_setter;
        let actual_body_velocity = *body.get_linear_velocity();
        let mass = body.get_mass();
        let impulse = Vec2::new(mass * (desired_horizontal_velocity - actual_body_velocity.x), 0.0);
        let body_center = *body.get_world_center();
        body.apply_linear_impulse(&impulse, &body_center, true);
    }

    fn turn_face(&mut self, direction: LrDirection) {
        self.facing_dir = direction;
    }
}
