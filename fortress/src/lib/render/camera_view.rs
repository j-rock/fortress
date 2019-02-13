use crate::render::Viewport;
use glm;

pub struct CameraView {
    pub eye: glm::Vec2,
    pub scale: glm::Vec2,
    pub viewport: Viewport,
}
