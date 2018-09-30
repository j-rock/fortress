use app::StatusOr;
use file::{
    ConfigWatcher,
    SimpleConfigManager,
};
use glm;
use render::CameraConfig;

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

    pub fn projection(&self, screen_size: glm::IVec2, scale: glm::Vec2) -> glm::Mat4 {
        let config = self.config_manager.get();
        let right = config.zoom / 2.0;
        let left = -right;
        let top = config.zoom * (screen_size.y as f32) / (2.0 * screen_size.x as f32);
        let bottom = -top;
        Self::ortho(scale.x * left, scale.x * right, scale.y * bottom, scale.y * top, config.z_near, config.z_far)
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

    pub fn view(&self, position: glm::Vec2) -> glm::Mat4 {
        let pos3d = glm::Vec3::new(position.x, position.y, 0.0);
        glm::ext::look_at(pos3d, pos3d + glm::vec3(0.0, 0.0, -1.0), glm::vec3(0.0, 1.0, 0.0))
    }

    pub fn update(&mut self) {
        self.config_manager.update();
    }
}
