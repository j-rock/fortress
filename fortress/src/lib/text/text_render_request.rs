use crate::text::{
    NamedText,
    RasterSize,
};
use glm;

pub enum TextContent {
    Number(i64),
    Text(NamedText)
}

pub struct TextRenderRequest {
    pub content: TextContent,
    // Specifies bottom-left corner of first character.
    // X in [0.0, 1.0) which goes from left to right.
    // Y in [0.0, 1.0) which goes from bottom to top.
    // Z in [0.0, 1.0) where 0.0 is closest to camera.
    pub screen_position_percentage: glm::Vec3,
    pub raster_size: RasterSize,
    pub color: glm::Vec3,
    pub alpha: f32,
}
