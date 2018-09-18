use gl;
use glm;

#[derive(Debug)]
pub struct Viewport {
    pub bottom_left: glm::IVec2,
    pub viewport_size: glm::IVec2,
}

impl Viewport {
    pub fn default(screen_size: &glm::IVec2) -> Viewport {
        Viewport {
            bottom_left: glm::ivec2(0,0),
            viewport_size: screen_size.clone(),
        }
    }

    pub fn set(&self) {
        unsafe {
            gl::Viewport(self.bottom_left.x, self.bottom_left.y, self.viewport_size.x, self.viewport_size.y);
        }
    }
}