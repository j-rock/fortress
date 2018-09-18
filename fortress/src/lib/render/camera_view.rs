use glm;
use render::Viewport;

pub struct CameraView {
    pub eye: glm::Vec2,
    pub scale: glm::Vec2,
    pub viewport: Viewport,
}
