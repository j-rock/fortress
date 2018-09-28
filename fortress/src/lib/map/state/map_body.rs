use entity::{
    Entity,
    EntityType,
    RegisteredBody,
};
use liquidfun::box2d::{
    collision::shapes::polygon_shape::PolygonShape,
    common::math::Vec2,
    dynamics::{
        body::BodyDef,
        fixture::FixtureDef,
    },
};
use map::{
    file::MapFile,
    Map,
    MapConfig,
};
use physics::{
    collision_category,
    PhysicsSimulation,
};

pub struct MapBody {
    pub wall_body: RegisteredBody,
}

impl MapBody {
    pub fn new(config: &MapConfig, map_file: &MapFile, physics_sim: &mut PhysicsSimulation) -> MapBody {
        let body_def = BodyDef::default();
        let wall_body = physics_sim.get_world_mut().create_body(&body_def);
        let mut poly_shape = PolygonShape::new();
        let cell_len = config.map_file_cell_length;
        for wall in map_file.walls.iter() {
            let (hx, hy) = (cell_len * (wall.size.0 as f32) / 2.0, cell_len * (wall.size.1 as f32) / 2.0);
            let top_left = wall.top_left_location.to_2d(cell_len);
            let center = Vec2::new(top_left.x + hx, top_left.y - hy);
            poly_shape.set_as_box_oriented(hx, hy, &center, 0.0);
            let mut fixture_def = FixtureDef::new(&poly_shape);
            fixture_def.filter.category_bits = collision_category::BARRIER;
            fixture_def.filter.mask_bits = collision_category::MASK_ALLOW_ALL;
            fixture_def.friction = config.friction;
            wall_body.create_fixture(&fixture_def);
        }

        MapBody {
            wall_body: RegisteredBody::new(wall_body, physics_sim.registrar(), None),
        }
    }

    pub fn register(&mut self, map: *const Map) {
        let wall_entity = Entity::new(EntityType::Wall, map);
        self.wall_body.register(wall_entity);
    }
}
