use entity::{
    Entity,
    EntityRegistrar,
    EntityType,
    RegisteredBody,
};
use liquidfun::box2d::{
    collision::shapes::polygon_shape::PolygonShape,
    common::math::Vec2,
    dynamics::{
        body::BodyDef,
        fixture::FixtureDef,
        world::World,
    },
};
use map::{
    Map,
    MapConfig,
};
use physics::collision_category;

pub struct MapBody {
    pub platform_body: RegisteredBody,
}

impl MapBody {
    pub fn new(config: &MapConfig, registrar: EntityRegistrar, world: &mut World) -> MapBody {
        let body_def = BodyDef::default();
        let platform_body = world.create_body(&body_def);
        let mut poly_shape = PolygonShape::new();
        for platform in config.platforms.iter() {
            let (hx, hy) = (platform.width as f32 / 2.0, platform.height as f32 / 2.0);
            let center = Vec2::new(platform.top_left_x as f32 + hx, platform.top_left_y as f32 - hy);
            poly_shape.set_as_box_oriented(hx, hy, &center, 0.0);
            let mut fixture_def = FixtureDef::new(&poly_shape);
            fixture_def.filter.category_bits = collision_category::BARRIER;
            fixture_def.filter.mask_bits = collision_category::MASK_ALLOW_ALL;
            fixture_def.friction = config.friction;
            platform_body.create_fixture(&fixture_def);
        }

        MapBody {
            platform_body: RegisteredBody::new(platform_body, registrar, None),
        }
    }

    pub fn register(&mut self, map: *const Map) {
        let platform_entity = Entity::new(EntityType::Platform, map);
        self.platform_body.register(platform_entity);
    }
}
