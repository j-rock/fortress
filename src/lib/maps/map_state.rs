use crate::{
    dimensions::GridIndex,
    maps::{
        MapCell,
        MapConfig,
        MapFile,
        state::MapBody,
    },
    physics::PhysicsSimulation,
    render::hex_renderer::{
        HexData,
        HexRenderer,
    }
};
use hashbrown::HashMap;

pub struct MapState {
    pub cells: HashMap<GridIndex, MapCell>,
    pub body: MapBody,
    hex_cell_length: f64,
}

impl MapState {
    pub fn new(config: &MapConfig, map_file: &MapFile, physics_sim: &mut PhysicsSimulation) -> MapState {
        let cells: HashMap<_, _> = map_file.cells
            .iter()
            .map(|(grid_index, map_file_cell)| {
                (*grid_index, MapCell::from_map_file_cell(map_file_cell))
            })
            .collect();

        let body = MapBody::new(config, &cells, physics_sim);

        MapState {
            cells,
            body,
            hex_cell_length: config.cell_length
        }
    }

    pub fn draw(&self, renderer: &mut HexRenderer) {
        let mut data = Vec::with_capacity(self.cells.len());
        for (grid_index, map_cell) in self.cells.iter() {
            data.push(HexData {
                position: *grid_index,
                height: map_cell.height,
                top_y_coord: map_cell.top_y_coord,
                rgba_color: map_cell.rgba_color
            });
        }
        renderer.queue(self.hex_cell_length, &data);
    }
}
