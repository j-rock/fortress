use crate::maps::MapFileCell;

#[derive(Clone, Deserialize)]
pub struct MapCell {
}

impl MapCell {
    pub fn from_map_file_cell(cell: &MapFileCell) -> MapCell {
        MapCell {
        }
    }
}