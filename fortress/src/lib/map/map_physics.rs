use entity::{
    EntityRegistrar,
    EntityType,
    Registered
};
use liquidfun::box2d::{
    collision::shapes::polygon_shape::PolygonShape,
    common::math::Vec2,
    dynamics::{
        body::{
            Body,
            BodyDef,
        },
        fixture::FixtureDef,
        world::World,
    },
};
use map::{
    Map,
    MapConfig,
};
use physics::PhysicsSimulation;


pub struct MapPhysics {
    platform_body: Registered<Body>,
}

impl MapPhysics {
    pub fn new(config: &MapConfig, physics_sim: &mut PhysicsSimulation) -> MapPhysics {
        let platform_body = Self::create_body_from_platforms(config, physics_sim.get_world_mut());

        MapPhysics {
            platform_body: Self::registered_platform(platform_body)
        }
    }

    pub fn update(&mut self, registrar: &mut EntityRegistrar, map: *const Map) {
        self.platform_body.register(registrar, map);
    }

    pub fn get_platform_body_mut(&mut self) -> &mut Body {
        &mut self.platform_body.data
    }

    pub fn redeploy(&mut self, config: &MapConfig, registrar: &mut EntityRegistrar) {
        self.platform_body.unregister(registrar);
        let mut world = self.platform_body.data.get_world();
        world.destroy_body(&mut self.platform_body.data);
        let platform_body = Self::create_body_from_platforms(config, &mut world);
        self.platform_body = Self::registered_platform(platform_body);
    }

    fn registered_platform(body: Body) -> Registered<Body> {
        Registered::new(body, EntityType::Platform)
    }

    fn create_body_from_platforms(config: &MapConfig, world: &mut World) -> Body {
        let body_def = BodyDef::default();
        let platform_body = world.create_body(&body_def);
        let mut poly_shape = PolygonShape::new();
        for platform in config.platforms.iter() {
            let (hx, hy) = (platform.width as f32 / 2.0, platform.height as f32 / 2.0);
            let center = Vec2::new(platform.top_left_x as f32 + hx, platform.top_left_y as f32 - hy);
            poly_shape.set_as_box_oriented(hx, hy, &center, 0.0);
            let fixture_def = FixtureDef::new(&poly_shape);
            platform_body.create_fixture(&fixture_def);
        }
        platform_body
    }
}