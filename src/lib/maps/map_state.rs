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
use hashbrown::{
    HashMap,
    HashSet,
};

pub struct MapState {
    cells: HashMap<GridIndex, MapCell>,
    _spawns: HashSet<GridIndex>,
    _body: MapBody,
    hex_cell_length: f64,
}

impl MapState {
    pub fn new(config: &MapConfig, map_file: &MapFile, physics_sim: &mut PhysicsSimulation) -> MapState {
        let cells: HashMap<_, _> = map_file.cells
            .iter()
            .map(|map_file_cell| {
                (map_file_cell.grid_index(), MapCell::from_map_file_cell(map_file_cell))
            })
            .collect();

        let spawns: HashSet<_> = map_file.cells
            .iter()
            .filter_map(|map_file_cell| {
                if !map_file_cell.is_spawn() {
                    return None;
                }
                return Some(map_file_cell.grid_index());
            })
            .collect();

        let body = MapBody::new(config, &cells, physics_sim);

        MapState {
            cells,
            _spawns: spawns,
            _body: body,
            hex_cell_length: config.cell_length
        }
    }

    pub fn draw(&self, renderer: &mut HexRenderer) {
        let mut data = Vec::with_capacity(self.cells.len());
        for (grid_index, map_cell) in self.cells.iter() {
            data.push(HexData {
                position: *grid_index,
                height: map_cell.height,
                elevation: map_cell.elevation,
                rgba_color: map_cell.rgba_color
            });
        }
        renderer.queue(self.hex_cell_length, &data);
    }
}
