use dimensions::{
    LrDirection,
    time::DeltaTime,
};
use entity::{
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
            Fixture,
            FixtureDef,
        },
        world::World,
    },
};
use physics::PhysicsSimulation;
use player::{
    Player,
    PlayerConfig,
    JumpTracker,
};

pub struct PlayerPhysics {
    player_body: Body,
    foot_sensor: Registered<Fixture>,
    jump_tracker: JumpTracker,
}

impl PlayerPhysics {
    pub fn new(config: &PlayerConfig, physics_sim: &mut PhysicsSimulation) -> PlayerPhysics {
        let (player_body, foot_sensor) = Self::create_body_from_config(config, physics_sim.get_world_mut());
        let foot_sensor = Self::registered_foot_sensor(foot_sensor);
        let jump_tracker = JumpTracker::new(config);

        PlayerPhysics {
            player_body,
            foot_sensor,
            jump_tracker,
        }
    }

    pub fn update(&mut self, dt: DeltaTime, registrar: &mut EntityRegistrar, player: *const Player) {
        self.foot_sensor.register(registrar, player);
        self.jump_tracker.update(dt);
    }

    pub fn get_position(&self) -> Vec2 {
        *self.player_body.get_position()
    }

    pub fn move_horizontal(&mut self, speed: f32, dir: Option<LrDirection>) {
        let desired_horizontal_velocity = speed * match dir {
            None => 0.0,
            Some(LrDirection::Left) => -1.0,
            Some(LrDirection::Right) => 1.0
        };

        let actual_body_velocity = *self.player_body.get_linear_velocity();
        let mass = self.player_body.get_mass();
        let impulse = Vec2::new(mass * (desired_horizontal_velocity - actual_body_velocity.x), 0.0);
        let body_center = *self.player_body.get_world_center();
        self.player_body.apply_linear_impulse(&impulse, &body_center, true);
    }

    pub fn jump(&mut self) {
        self.jump_tracker.try_jump(&self.player_body);
    }

    pub fn make_foot_contact(&mut self) {
        self.jump_tracker.make_foot_contact();
    }

    pub fn redeploy(&mut self, config: &PlayerConfig, registrar: &mut EntityRegistrar) {
        self.foot_sensor.unregister(registrar);

        let mut world = self.player_body.get_world();
        world.destroy_body(&mut self.player_body);

        let (player_body, foot_sensor_fixture) = Self::create_body_from_config(config, &mut world);
        self.player_body = player_body;
        self.foot_sensor = Self::registered_foot_sensor(foot_sensor_fixture);
        self.jump_tracker = JumpTracker::new(config);
    }

    fn registered_foot_sensor(fixture: Fixture) -> Registered<Fixture> {
        Registered::new(fixture, EntityType::PlayerFootSensor)
    }

    fn create_body_from_config(config: &PlayerConfig, world: &mut World) -> (Body, Fixture) {
        let mut body_def = BodyDef::default();
        body_def.body_type = BodyType::DynamicBody;
        body_def.position = Vec2::new(config.spawn_location.0 as f32, config.spawn_location.1 as f32);
        body_def.fixed_rotation = true;

        let player_body = world.create_body(&body_def);

        // Player body fixture
        let mut poly_shape = PolygonShape::new();
        let (hx, hy) = (config.size.0 as f32 / 2.0, config.size.1 as f32 / 2.0);
        poly_shape.set_as_box(hx, hy);

        let mut fixture_def = FixtureDef::new(&poly_shape);
        fixture_def.restitution = config.restitution;
        fixture_def.filter.category_bits = 0x0002;
        player_body.create_fixture(&fixture_def);

        // Foot sensor fixture
        let (hx, hy) = (config.foot_sensor_size.0 / 2.0, config.foot_sensor_size.1 / 2.0);
        let sensor_center = Vec2::new(config.foot_sensor_center.0, config.foot_sensor_center.1);
        poly_shape.set_as_box_oriented(hx, hy, &sensor_center, 0.0);
        fixture_def.filter.category_bits = 0x0001;
        fixture_def.filter.mask_bits = 0xFFFF & !0x0002; // Ignore player body.
        fixture_def.is_sensor = true;
        let foot_sensor = player_body.create_fixture(&fixture_def);

        (player_body, foot_sensor)
    }
}

