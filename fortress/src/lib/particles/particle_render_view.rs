use crate::render::attribute;
use glm;

pub struct ParticleRenderView<'a> {
    pub attr_pos: &'a mut Vec<Vec3Attr>,
    pub attr_color: &'a mut Vec<Vec3Attr>,
    pub attr_bloom: &'a mut Vec<BloomAttr>,
    pub attr_alpha: &'a mut Vec<FloatAttr>,
    pub attr_size: &'a mut Vec<FloatAttr>,
}

#[repr(C)]
pub struct Vec3Attr {
    val: glm::Vec3,
}

impl Vec3Attr {
    pub fn new(val: glm::Vec3) -> Vec3Attr {
        Vec3Attr {
            val
        }
    }
}

impl attribute::KnownComponent for Vec3Attr {
    fn component() -> (attribute::NumComponents, attribute::ComponentType) {
        (attribute::NumComponents::S3, attribute::ComponentType::Float)
    }
}

#[repr(C)]
pub struct FloatAttr {
    val: f32,
}

impl FloatAttr {
    pub fn new(val: f32) -> Self {
        FloatAttr {
            val
        }
    }
}

impl attribute::KnownComponent for FloatAttr {
    fn component() -> (attribute::NumComponents, attribute::ComponentType) {
        (attribute::NumComponents::S1, attribute::ComponentType::Float)
    }
}

#[repr(C)]
pub struct BloomAttr {
    color: glm::Vec3,
    intensity: f32,
}

impl BloomAttr {
    pub fn new(color: glm::Vec3, intensity: f32) -> Self {
        BloomAttr {
            color,
            intensity,
        }
    }
}

impl attribute::KnownComponent for BloomAttr {
    fn component() -> (attribute::NumComponents, attribute::ComponentType) {
        (attribute::NumComponents::S4, attribute::ComponentType::Float)
    }
}
