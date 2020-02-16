use crate::{
    dimensions::{
        GridDirection,
        GridIndex
    },
    entities::{
        Entity,
        RegisteredBody,
        RegisteredBodyBuilder,
    },
    maps::MapConfig,
    physics::{
        collision_category,
        PhysicsSimulation,
    }
};
use ncollide2d::{
    pipeline::object::CollisionGroups,
    shape::ShapeHandle,
};
use nphysics2d::object::{
    BodyStatus,
    ColliderDesc,
    RigidBodyDesc,
};
use std::collections::HashSet;

pub struct MapBody {
    pub wall_body: RegisteredBody,
}

impl MapBody {
    pub fn new(config: &MapConfig, terrain: &HashSet<GridIndex>, physics_sim: &mut PhysicsSimulation) -> MapBody {
        let rigid_body = RigidBodyDesc::new()
            .status(BodyStatus::Static)
            .build();

        let mut wall_body_builder = RegisteredBodyBuilder::new()
            .rigid_body(rigid_body)
            .entity(Entity::MapWall);

        let axial_to_cartesian = GridIndex::axial_to_cartesian(config.cell_length);
        for grid_index in terrain.iter() {
           for grid_dir in GridDirection::all() {
               if !terrain.contains(&grid_index.neighbor(*grid_dir)) {
                   let segment = grid_index.edge_line_segment(*grid_dir, config.cell_length, &axial_to_cartesian);
                   let collider_desc = ColliderDesc::new(ShapeHandle::new(segment))
                       .collision_groups(CollisionGroups::new()
                           .with_membership(&[collision_category::BARRIER])
                           .with_whitelist(collision_category::ALLOW_ALL_WHITELIST));
                   wall_body_builder.add_collider(collider_desc);
               }
           }
        }

        MapBody {
            wall_body: wall_body_builder.build(physics_sim)
        }
    }
}
