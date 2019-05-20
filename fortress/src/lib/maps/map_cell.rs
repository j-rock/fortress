use crate::maps::MapFileCell;

#[derive(Clone)]
pub struct MapCell {
    pub height: f32,
    pub elevation: f32,
}

impl MapCell {
    pub fn from_map_file_cell(cell: &MapFileCell) -> MapCell {
        MapCell {
            height: cell.height,
            elevation: cell.elevation,
        }
    }
}