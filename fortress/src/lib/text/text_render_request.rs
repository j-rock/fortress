use crate::text::{
    NamedText,
    TextSize,
};
use glm;
use nalgebra::Point3;

pub enum TextSurface {
    World(Point3<f64>),
    Screen(glm::Vec2)
}

pub struct RenderNumber {
    pub value: i64,
    pub size: TextSize,
    pub surface: TextSurface,
    pub color: glm::Vec3,
    pub alpha: f32,
}

pub struct RenderText {
    pub text: NamedText,
    pub size: TextSize,
    pub surface: TextSurface,
    pub color: glm::Vec3,
    pub alpha: f32,
}
