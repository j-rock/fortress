use dimensions::LrDirection;
use entity::{
    Entity,
    EntityType,
    RegisteredBody,
    RegisteredFixture,
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
    },
};
use physics::{
    collision_category,
    PhysicsSimulation,
};
use player::{
    Player,
    PlayerConfig,
};

pub struct PlayerBody {
    pub sword_size: Vec2,
    pub sword_offset_from_body: Vec2,
    pub sword_sensor: RegisteredFixture,

    pub facing_dir: LrDirection,

    // Declare last so we clean up fixtures first.
    pub body: RegisteredBody,
}

impl PlayerBody {
    pub fn new(config: &PlayerConfig, spawn: Vec2, physics_sim: &mut PhysicsSimulation) -> PlayerBody {
        let mut body_def = BodyDef::default();
        body_def.body_type = BodyType::DynamicBody;
        body_def.position = spawn;
        body_def.fixed_rotation = true;

        let body = physics_sim.get_world_mut().create_body(&body_def);

        // Player body fixture
        let mut poly_shape = PolygonShape::new();
        {
            let (hx, hy) = (config.size.0 as f32 / 2.0, config.size.1 as f32 / 2.0);
            poly_shape.set_as_box(hx, hy);

            let mut fixture_def = FixtureDef::new(&poly_shape);
            fixture_def.restitution = config.restitution;
            fixture_def.filter.category_bits = collision_category::PLAYER_BODY;
            fixture_def.filter.mask_bits = collision_category::BARRIER | collision_category::PICKUP;
            body.create_fixture(&fixture_def);
        }

        let (sword_size, sword_offset_from_body, sword_sensor) = {
            let sword_size = Vec2 {
                x: config.sword_sensor_size.0,
                y: config.sword_sensor_size.1,
            };
            let sensor_center = Vec2::new(config.sword_sensor_center.0, config.sword_sensor_center.1);
            let sword_sensor_fixture = Self::create_sword_sensor_fixture(sword_size, sensor_center, &body);
            let sword_sensor = RegisteredFixture::new(sword_sensor_fixture, physics_sim.registrar(), None);

            (sword_size, sensor_center, sword_sensor)
        };

        let body = RegisteredBody::new(body, physics_sim.registrar(), None);

        PlayerBody {
            sword_size,
            sword_offset_from_body,
            sword_sensor,
            facing_dir: LrDirection::Right,
            body,
        }
    }

    pub fn register(&mut self, player: *const Player) {
        let player_entity = Entity::new(EntityType::Player, player);
        self.body.register(player_entity);

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

        let body = &mut self.body.data_setter;

        let actual_body_velocity = *body.get_linear_velocity();
        let mass = body.get_mass();
        let impulse = Vec2::new(mass * (desired_horizontal_velocity - actual_body_velocity.x), 0.0);
        let body_center = *body.get_world_center();
        body.apply_linear_impulse(&impulse, &body_center, true);
    }

    pub fn body(&self) -> &Body {
        &self.body.data_setter
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
            let sword_sensor_fixture = Self::create_sword_sensor_fixture(self.sword_size, self.sword_offset_from_body, &self.body.data_setter);
            self.sword_sensor = RegisteredFixture::new(sword_sensor_fixture, registrar, entity);
        }
    }

    pub fn enable_sword_collision(&mut self) {
        let mut collision_filter = Self::disabled_sword_collision_filter();
        collision_filter.mask_bits = collision_category::WRAITH | collision_category::INTERACT;
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
