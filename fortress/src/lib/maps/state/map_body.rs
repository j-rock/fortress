use crate::{
    dimensions::{
        GridDirection,
        GridIndex
    },
    entities::{
        Entity,
        RegisteredBody,
    },
    maps::MapConfig,
    physics::{
        collision_category,
        PhysicsSimulation,
    }
};
use ncollide2d::{
    shape::ShapeHandle,
    world::CollisionGroups
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
        let mut body_desc = RigidBodyDesc::new()
            .status(BodyStatus::Static);

        let mut collider_descs = Vec::new();
        let axial_to_cartesian = GridIndex::axial_to_cartesian(config.cell_length);
        for grid_index in terrain.iter() {
           for grid_dir in GridDirection::all() {
               if !terrain.contains(&grid_index.neighbor(*grid_dir)) {
                   let segment = grid_index.edge_line_segment(*grid_dir, config.cell_length, &axial_to_cartesian);
                   let collider_desc = ColliderDesc::new(ShapeHandle::new(segment))
                       .collision_groups(CollisionGroups::new()
                           .with_membership(&[collision_category::BARRIER])
                           .with_whitelist(collision_category::ALLOW_ALL_WHITELIST));
                   collider_descs.push(collider_desc);
               }
           }
        }

        for collider_desc in collider_descs.iter() {
            body_desc.add_collider(collider_desc);
        }

        let body_handle = body_desc
            .build(physics_sim.borrow_mut().world_mut())
            .handle();

        MapBody {
            wall_body: RegisteredBody::new(body_handle, Entity::MapWall, physics_sim),
        }
    }
}
