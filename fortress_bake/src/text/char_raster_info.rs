use glm;

pub struct CharRasterInfo {
    pub raster_dimensions: glm::Vec2,
    pub advance_width: f32,
    pub left_side_bearing: f32,
    pub height_offset: f32,
}