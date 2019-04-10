use crate::file;
use std::path::PathBuf;

#[derive(Clone, Deserialize)]
pub struct MapConfig {
    pub cell_length: f64,
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

