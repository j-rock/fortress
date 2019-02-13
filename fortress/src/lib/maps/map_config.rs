use crate::file;
use std::path::PathBuf;

#[derive(Clone, Deserialize)]
pub struct MapConfig {
    pub map_file_cell_length: f32,
    pub friction: f32,
    pub map_label: MapLabel,
}

#[derive(Copy, Clone, Deserialize)]
pub enum MapLabel {
    Dungeon
}

impl MapLabel {
    pub fn to_path(self) -> PathBuf {
        let map_name = match self {
            MapLabel::Dungeon => "dungeon.map"
        };
        file::util::resource_path("maps", map_name)
    }
}

