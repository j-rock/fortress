use crate::text::{
    NamedText,
    RasterSize,
};
use glm;
use nalgebra::Point3;

pub enum TextContent {
    Number(i64),
    Text(NamedText)
}

pub enum TextSurface {
    World(Point3<f64>),
    Screen(glm::Vec3)
}

pub struct TextRenderRequest {
    pub surface: TextSurface,
    pub content: TextContent,
    pub raster_size: RasterSize,
    pub draw_size: f32,
    pub color: glm::Vec3,
    pub alpha: f32,
}
