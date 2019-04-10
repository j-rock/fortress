use crate::{
    dimensions::{
        GridDirection,
        GridIndex
    },
    entities::{
        Entity,
        RegisteredBody,
    },
    maps::{
        Map,
        MapCell,
        MapFile,
        MapConfig,
    },
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
use hashbrown::HashMap;

pub struct MapBody {
    pub wall_body: RegisteredBody,
    pub cells: HashMap<GridIndex, MapCell>
}

impl MapBody {
    pub fn new(config: &MapConfig, map_file: &MapFile, physics_sim: &mut PhysicsSimulation) -> MapBody {
        let cells: HashMap<_, _> = map_file.cells
            .iter()
            .map(|&(grid_cell, map_file_cell)| {
                (grid_cell, MapCell::from_map_file_cell(map_file_cell))
            })
            .collect();

        let body_desc = RigidBodyDesc::new()
            .status(BodyStatus::Static);

        for &(grid_index, _) in cells.iter() {
           for grid_dir in GridDirection::all() {
               if !cells.contains_key(grid_index.neighbor(grid_dir)) {
                   let segment = grid_index.edge_line_segment(grid_dir, config.cell_length);
                   let collider_desc = ColliderDesc::new(ShapeHandle::new(segment))
                       .collision_groups(CollisionGroups::new()
                           .with_membership(&[collision_category::BARRIER])
                           .with_whitelist(collision_category::ALLOW_ALL_WHITELIST));
                   body_desc.add_collider(collider_desc);
               }
           }
        }

        let body_handle = body_desc
            .build(physics_sim.borrow_mut().world_mut())
            .handle();

        MapBody {
            wall_body: RegisteredBody::new(body_handle, Entity::MapWall, physics_sim),
            cells
        }
    }
}
