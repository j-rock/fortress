use crate::dimensions::GridIndex;

#[derive(Clone, Deserialize)]
pub struct MapFileCell {
    pub height: f32,
    pub top_y_coord: f32,
    pub rgba_color: (f32, f32, f32, f32)
}

#[derive(Clone, Deserialize)]
pub struct MapFile {
    pub cells: Vec<(GridIndex, MapFileCell)>
}