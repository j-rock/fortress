use dimensions::LrDirection;
use entity::{
    Entity,
    EntityRegistrar,
    EntityType,
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
            Filter,
            Fixture,
            FixtureDef
        },
        world::World
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

    pub sword_size: Vec2,
    pub sword_offset_from_body: Vec2,
    pub sword_sensor: Registered<Fixture>,

    pub facing_dir: LrDirection,
}

impl PlayerBody {
    pub fn new(config: &PlayerConfig, registrar: &EntityRegistrar, world: &mut World) -> PlayerBody {
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
            fixture_def.filter.mask_bits = collision_category::MASK_ALLOW_ALL & !collision_category::WRAITH;
            body.create_fixture(&fixture_def);
        }

        let foot_sensor = {
            let (hx, hy) = (config.foot_sensor_size.0 / 2.0, config.foot_sensor_size.1 / 2.0);
            let sensor_center = Vec2::new(config.foot_sensor_center.0, config.foot_sensor_center.1);
            poly_shape.set_as_box_oriented(hx, hy, &sensor_center, 0.0);

            let mut fixture_def = FixtureDef::new(&poly_shape);
            fixture_def.filter.category_bits = collision_category::PLAYER_BODY;
            fixture_def.filter.mask_bits = collision_category::MASK_ALLOW_ALL & !collision_category::WRAITH;
            fixture_def.is_sensor = true;

            let foot_sensor_fixture = body.create_fixture(&fixture_def);
            Registered::new(foot_sensor_fixture, registrar.clone(), None)
        };

        let (sword_size, sword_offset_from_body, sword_sensor) = {
            let sword_size = Vec2 {
                x: config.sword_sensor_size.0,
                y: config.sword_sensor_size.1,
            };
            let sensor_center = Vec2::new(config.sword_sensor_center.0, config.sword_sensor_center.1);
            let sword_sensor_fixture = Self::create_sword_sensor_fixture(sword_size, sensor_center, &body);
            let sword_sensor = Registered::new(sword_sensor_fixture, registrar.clone(), None);

            (sword_size, sensor_center, sword_sensor)
        };

        PlayerBody {
            body,
            foot_sensor,
            sword_size,
            sword_offset_from_body,
            sword_sensor,
            facing_dir: LrDirection::Right,
        }
    }

    pub fn register(&mut self, player: *const Player) {
        let foot_sensor_entity = Entity::new(EntityType::PlayerFootSensor, player);
        self.foot_sensor.register(foot_sensor_entity);

        let sword_sensor_entity = Entity::new(EntityType::PlayerSwordSensor, player);
        self.sword_sensor.register(sword_sensor_entity);
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

    pub fn body(&self) -> &Body {
        &self.body
    }

    pub fn turn_face(&mut self, direction: LrDirection) {
        if self.facing_dir != direction {
            self.facing_dir = direction;

            self.sword_offset_from_body = Vec2 {
                x: -self.sword_offset_from_body.x,
                y: self.sword_offset_from_body.y
            };

            let registrar = self.sword_sensor.registrar.clone();
            let entity = self.sword_sensor.entity.clone();
            let sword_sensor_fixture = Self::create_sword_sensor_fixture(self.sword_size, self.sword_offset_from_body, &self.body);
            self.sword_sensor = Registered::new(sword_sensor_fixture, registrar, entity);
        }
    }

    pub fn enable_sword_collision(&mut self) {
        let mut collision_filter = Self::disabled_sword_collision_filter();
        collision_filter.mask_bits = collision_category::WRAITH;
        self.sword_sensor.data_setter.set_filter_data(&collision_filter);
    }

    pub fn disable_sword_collision(&mut self) {
        let collision_filter = Self::disabled_sword_collision_filter();
        self.sword_sensor.data_setter.set_filter_data(&collision_filter);
    }

    fn create_sword_sensor_fixture(size: Vec2, body_position_offset: Vec2, body: &Body) -> Fixture {
        let mut poly_shape = PolygonShape::new();
        poly_shape.set_as_box_oriented(size.x / 2.0, size.y / 2.0, &body_position_offset, 0.0);

        let mut fixture_def = FixtureDef::new(&poly_shape);
        fixture_def.filter = Self::disabled_sword_collision_filter();
        fixture_def.is_sensor = true;

        body.create_fixture(&fixture_def)
    }

    fn disabled_sword_collision_filter() -> Filter {
        let mut filter = Filter::default();
        filter.category_bits = collision_category::PLAYER_WEAPON;
        filter.mask_bits = collision_category::MASK_ALLOW_NONE;
        filter
    }
}

impl Drop for PlayerBody {
    fn drop(&mut self) {
        let mut world = self.body.get_world();
        world.destroy_body(&mut self.body);
    }
}