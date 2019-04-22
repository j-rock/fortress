use crate::{
    app::StatusOr,
    file::{
        ConfigWatcher,
        SimpleConfigManager,
    },
    render::CameraConfig
};
use glm;

pub struct Camera {
    config_manager: SimpleConfigManager<CameraConfig>,
}

impl Camera {
    pub fn new(config_watcher: &mut ConfigWatcher) -> StatusOr<Camera> {
        let config_manager: SimpleConfigManager<CameraConfig> = SimpleConfigManager::from_config_resource(config_watcher, "camera.conf")?;
        Ok(Camera {
            config_manager,
        })
    }

    pub fn projection(&self, screen_size: glm::IVec2) -> glm::Mat4 {
        let config = self.config_manager.get();
        let right = 1.0 / (2.0 * config.zoom);
        let left = -right;
        let top = (screen_size.y as f32) / (2.0 * config.zoom * screen_size.x as f32);
        let bottom = -top;
        Self::ortho(left, right, bottom, top, config.z_near, config.z_far)
    }

    fn ortho(left: f32, right: f32, bottom: f32, top: f32, z_near: f32, z_far: f32) -> glm::Mat4 {
        let rml = right - left;
        let tmb = top - bottom;
        let fmn = z_far - z_near;

        glm::Matrix4::new(
            glm::vec4(2.0 / rml, 0.0, 0.0, 0.0),
            glm::vec4(0.0, 2.0 / tmb, 0.0, 0.0),
            glm::vec4(0.0, 0.0, -2.0 / fmn, 0.0),
            glm::vec4(-(right + left) / rml, -(top + bottom) / tmb, -(z_far + z_near) / fmn, 1.0))
    }

    pub fn view(&self, lookat: glm::Vec3, up: glm::Vec3) -> glm::Mat4 {
        let config = self.config_manager.get();
        let position = glm::vec3(config.position.0, config.position.1, config.position.2);
        glm::ext::look_at(position, position + lookat, up)
    }

    pub fn lookat_right_and_up(&self) -> (glm::Vec3, glm::Vec3, glm::Vec3) {
        let config = self.config_manager.get();
        let lookat = glm::builtin::normalize(glm::vec3(config.lookat.0, config.lookat.1, config.lookat.2));
        let right = glm::builtin::normalize(glm::vec3(config.right.0, config.right.1, config.right.2));
        let up = glm::builtin::cross(right, lookat);
        (lookat, right, up)
    }

    pub fn update(&mut self) {
        self.config_manager.update();
    }
}
