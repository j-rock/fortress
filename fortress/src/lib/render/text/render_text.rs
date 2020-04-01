use crate::render::{
    TextKey,
};
use glm;

pub struct RenderText {
    pub key: TextKey,
    pub screen_position: glm::Vec2,
    pub color: glm::Vec4,
}
