use crate::maps::MapFileCell;

#[derive(Clone)]
pub struct MapCell {
    pub height: f32,
    pub top_y_coord: f32,
    pub rgba_color: glm::Vec4,
}

impl MapCell {
    pub fn from_map_file_cell(cell: &MapFileCell) -> MapCell {
        MapCell {
            height: cell.height,
            top_y_coord: cell.top_y_coord,
            rgba_color: glm::vec4(cell.rgba_color.0,
                                  cell.rgba_color.1,
                                  cell.rgba_color.2,
                                 cell.rgba_color.3)
        }
    }
}