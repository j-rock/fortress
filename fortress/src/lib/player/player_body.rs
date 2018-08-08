use dimensions::LrDirection;
use entity::{
    Entity,
    EntityType,
    EntityRegistrar,
    Registered,
};
use liquidfun::box2d::{
    collision::shapes::polygon_shape::PolygonShape,
    common::math::Vec2,
    dynamics::{
        body::{
            Body,
            BodyDef,
            BodyType,
        },
        fixture::{
            Fixture,
            FixtureDef
        },
        world::World,
    },
};
use physics::collision_category;
use player::{
    Player,
    PlayerConfig,
};

pub struct PlayerBody {
    pub body: Body,
    pub foot_sensor: Registered<Fixture>,
}

impl PlayerBody {
    pub fn new(config: &PlayerConfig, registrar: EntityRegistrar, world: &mut World) -> PlayerBody {
        let mut body_def = BodyDef::default();
        body_def.body_type = BodyType::DynamicBody;
        body_def.position = Vec2::new(config.spawn_location.0 as f32, config.spawn_location.1 as f32);
        body_def.fixed_rotation = true;

        let body = world.create_body(&body_def);

        // Player body fixture
        let mut poly_shape = PolygonShape::new();
        {
            let (hx, hy) = (config.size.0 as f32 / 2.0, config.size.1 as f32 / 2.0);
            poly_shape.set_as_box(hx, hy);

            let mut fixture_def = FixtureDef::new(&poly_shape);
            fixture_def.restitution = config.restitution;
            fixture_def.filter.category_bits = collision_category::PLAYER_BODY;
            body.create_fixture(&fixture_def);
        }

        // Foot sensor
        let (hx, hy) = (config.foot_sensor_size.0 / 2.0, config.foot_sensor_size.1 / 2.0);
        let sensor_center = Vec2::new(config.foot_sensor_center.0, config.foot_sensor_center.1);
        poly_shape.set_as_box_oriented(hx, hy, &sensor_center, 0.0);

        let mut fixture_def = FixtureDef::new(&poly_shape);
        fixture_def.filter.category_bits = collision_category::COLLIDE_ALL;
        fixture_def.filter.mask_bits = collision_category::MASK_ALLOW_ALL & !collision_category::PLAYER_BODY;
        fixture_def.is_sensor = true;

        let foot_sensor_fixture = body.create_fixture(&fixture_def);
        let foot_sensor = Registered::new(foot_sensor_fixture, registrar, None);

        PlayerBody {
            body,
            foot_sensor,
        }
    }

    pub fn register(&mut self, player: *const Player) {
        let foot_sensor_entity = Entity::new(EntityType::PlayerFootSensor, player);
        self.foot_sensor.register(foot_sensor_entity);
    }

    pub fn move_horizontal(&mut self, speed: f32, dir: Option<LrDirection>) {
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

    pub fn body(&self) -> &Body {
        &self.body
    }
}

impl Drop for PlayerBody {
    fn drop(&mut self) {
        let mut world = self.body.get_world();
        world.destroy_body(&mut self.body);
    }
}