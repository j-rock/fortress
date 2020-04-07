use crate::text::{
    NamedText,
    TextSize,
};
use glm;
use nalgebra::Point3;

pub enum TextContent {
    Number(i64),
    Text(NamedText)
}

pub enum TextSurface {
    World(Point3<f64>),
    Screen(glm::Vec2)
}

pub struct TextRenderRequest {
    pub content: TextContent,
    pub size: TextSize,
    pub surface: TextSurface,
    pub color: glm::Vec3,
    pub alpha: f32,
}
