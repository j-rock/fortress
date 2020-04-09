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
    pub screen_position: glm::Vec3,
    pub content: TextContent,
    pub raster_size: RasterSize,
    pub draw_size: f32,
    pub color: glm::Vec3,
    pub alpha: f32,
}
