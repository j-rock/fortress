use entity::{
    Entity,
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
    pub fn new(config: &MapConfig, registrar: EntityRegistrar, physics_sim: &mut PhysicsSimulation) -> MapPhysics {
        let platform_body = Self::create_body_from_platforms(config, physics_sim.get_world_mut());

        MapPhysics {
            platform_body: Registered::new(platform_body, registrar, None),
        }
    }

    pub fn register(&mut self, map: *const Map) {
        let platform_entity = Entity::new(EntityType::Platform, map);
        self.platform_body.register(platform_entity);
    }

    pub fn get_platform_body_mut(&mut self) -> &mut Body {
        &mut self.platform_body.data_setter
    }

    pub fn redeploy(&mut self, config: &MapConfig) {
        let mut world = self.platform_body.data_setter.get_world();
        world.destroy_body(&mut self.platform_body.data_setter);

        let platform_body = Self::create_body_from_platforms(config, &mut world);
        let platform_entity = self.platform_body.entity;
        self.platform_body = Registered::new(platform_body, self.platform_body.registrar.clone(), platform_entity);
    }

    fn create_body_from_platforms(config: &MapConfig, world: &mut World) -> Body {
        let body_def = BodyDef::default();
        let platform_body = world.create_body(&body_def);
        let mut poly_shape = PolygonShape::new();
        for platform in config.platforms.iter() {
            let (hx, hy) = (platform.width as f32 / 2.0, platform.height as f32 / 2.0);
            let center = Vec2::new(platform.top_left_x as f32 + hx, platform.top_left_y as f32 - hy);
            poly_shape.set_as_box_oriented(hx, hy, &center, 0.0);
            let mut fixture_def = FixtureDef::new(&poly_shape);
            fixture_def.friction = config.friction;
            platform_body.create_fixture(&fixture_def);
        }
        platform_body
    }
}