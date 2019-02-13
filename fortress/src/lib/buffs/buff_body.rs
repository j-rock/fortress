use crate::{
    buffs::{
        BuffBox,
        BuffBoxConfig,
        BuffBoxPlacement,
        BuffDropConfig,
    },
    entities::{
        Entity,
        EntityType,
        RegisteredBody
    },
    physics::{
        collision_category,
        PhysicsSimulation,
    }
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
    },
};

pub struct BuffBody {
    pub buff_box_body: RegisteredBody,
    pub buff_drop_body: Option<RegisteredBody>
}

impl BuffBody {
    pub fn new(config: &BuffBoxConfig, placement: &BuffBoxPlacement, physics_sim: &mut PhysicsSimulation) -> BuffBody {
        let mut body_def = BodyDef::default();
        body_def.body_type = BodyType::DynamicBody;
        body_def.position = placement.location;
        body_def.fixed_rotation = true;
        let body = physics_sim.get_world_mut().create_body(&body_def);

        let mut poly_shape = PolygonShape::new();
        let (hx, hy) = (config.size.0 / 2.0, config.size.1 / 2.0);
        poly_shape.set_as_box(hx, hy);

        let mut fixture_def = FixtureDef::new(&poly_shape);
        fixture_def.density = config.density;
        fixture_def.friction = config.friction;
        fixture_def.filter.category_bits = collision_category::INTERACT;
        fixture_def.filter.mask_bits = collision_category::BARRIER | collision_category::PLAYER_WEAPON;

        body.create_fixture(&fixture_def);
        let buff_box_body = RegisteredBody::new(body, physics_sim.registrar(), None);

        BuffBody {
            buff_box_body,
            buff_drop_body: None,
        }
    }

    pub fn get_drop_body_position(&self) -> Option<(f32, f32)> {
        if let Some(ref drop_body) = self.buff_drop_body {
            let pos = drop_body.data_setter.get_position();
            Some((pos.x, pos.y))
        } else {
            None
        }
    }

    pub fn register(&mut self, buff_box: *const BuffBox) {
        let entity = Entity::new(EntityType::BuffBox, buff_box);
        self.buff_box_body.register(entity);
    }

    pub fn launch_drop(&mut self, config: &BuffDropConfig, entity: Entity, physics_sim: &mut PhysicsSimulation) {
        let buff_box_pos = self.buff_box_body.data_setter.get_position();
        let start_position = Vec2::new(config.start_position.0 + buff_box_pos.x, config.start_position.1 + buff_box_pos.y);

        let mut body_def = BodyDef::default();
        body_def.body_type = BodyType::DynamicBody;
        body_def.position = start_position;
        body_def.linear_velocity = Vec2::new(config.velocity.0, config.velocity.1);
        body_def.fixed_rotation = true;
        let body = physics_sim.get_world_mut().create_body(&body_def);

        let mut poly_shape = PolygonShape::new();
        let (hx, hy) = (config.size.0 / 2.0, config.size.1 / 2.0);
        poly_shape.set_as_box(hx, hy);

        let mut fixture_def = FixtureDef::new(&poly_shape);
        fixture_def.density = config.density;
        fixture_def.friction = config.friction;
        fixture_def.restitution = config.restitution;
        fixture_def.filter.category_bits = collision_category::PICKUP;
        fixture_def.filter.mask_bits = collision_category::BARRIER | collision_category::PLAYER_BODY;

        body.create_fixture(&fixture_def);
        self.buff_drop_body = Some(RegisteredBody::new(body, physics_sim.registrar(), Some(entity)));
    }

    pub fn destroy_drop(&mut self) {
        self.buff_drop_body = None;
    }
}
