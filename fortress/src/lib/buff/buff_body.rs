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
        fixture::FixtureDef,
        world::World,
    },
};
use physics::{
    collision_category,
    PhysicsSimulation,
};

pub struct BuffBody {
    pub half_size: (f32, f32),
    pub buff_box_body: Registered<Body>,
    pub buff_body: Option<Registered<Body>>
}

impl BuffBody {
    pub fn new(config: &BuffConfig, placement: &BuffBoxPlacement, physics_sim: &mut PhysicsSimulation) -> BuffBody {
        let mut body_def = BodyDef::default();
        body_def.body_type = BodyType::DynamicBody;
        body_def.position = Vec2::new(placement.location.0, placement.location.1);
        body_def.fixed_rotation = true;

        let body = physics_sim.get_world_mut().create_body(&body_def);
        let mut poly_shape = PolygonShape::new();
        let half_size = {
            let (hx, hy) = (config.buff_box_size.0 / 2.0, config.buff_box_size.1 / 2.0);
            poly_shape.set_as_box(hx, hy);

            let mut fixture_def = FixtureDef::new(&poly_shape);
            fixture_def.density = config.density;
            fixture_def.friction = config.friction;
            fixture_def.filter.category_bits = collision_category::INTERACT;
            fixture_def.filter.mask_bits = collision_category::MASK_ALLOW_ALL;
            body.create_fixture(&fixture_def);

            (hx, hy)
        };

        let buff_box_body = Registered::new(body, physics_sim.registrar(), None);
        BuffBody {
            buff_box_body,
            half_size,
            buff_body: None,
        }
    }

    pub fn register(&mut self, buff_box: *const BuffBox) {
        let entity = Entity::new(EntityType::BuffBox, buff_box);
        self.buff_box_body.register(entity);
    }
}