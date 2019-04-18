use crate::render::attribute;
use glm;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Texel {
    pub bottom_left: glm::Vec2,
    pub top_right: glm::Vec2,
}

impl attribute::KnownComponent for Texel {
    fn component() -> (attribute::NumComponents, attribute::ComponentType) {
        (attribute::NumComponents::S4, attribute::ComponentType::Float)
    }
}
