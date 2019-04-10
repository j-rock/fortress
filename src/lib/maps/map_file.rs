use crate::dimensions::GridIndex;

#[derive(Clone, Deserialize)]
pub struct MapFileCell {
}

#[derive(Clone, Deserialize)]
pub struct MapFile {
    pub cells: Vec<(GridIndex, MapFileCell)>
}