use app::StatusOr;
use camera::CameraConfig;
use file::{
    ConfigWatcher,
    SimpleConfigManager,
};
use glm;

pub struct Camera {
    config_manager: SimpleConfigManager<CameraConfig>,
    position: (f32, f32)
}

impl Camera {
    pub fn new(config_watcher: &mut ConfigWatcher) -> StatusOr<Camera> {
        let config_manager: SimpleConfigManager<CameraConfig> = SimpleConfigManager::new(config_watcher, "camera.conf")?;
        let position = config_manager.get().position;
        Ok(Camera {
            config_manager,
            position,
        })
    }

    pub fn projection(&self) -> glm::Mat4 {
        let config = self.config_manager.get();
        Self::ortho(config.left, config.right, config.bottom, config.top, config.z_near, config.z_far)
    }

    pub fn recenter(&mut self, pos: (f32, f32)) {
        self.position = pos;
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

    pub fn view(&self) -> glm::Mat4 {
        let pos3d = glm::Vec3::new(self.position.0, self.position.1, 0.0);
        glm::ext::look_at(pos3d, pos3d + glm::vec3(0.0, 0.0, -1.0), glm::vec3(0.0, 1.0, 0.0))
    }

    pub fn update(&mut self) {
        self.config_manager.update();
    }
}
