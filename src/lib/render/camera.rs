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

    pub fn perspective_projection(&self, screen_size: glm::IVec2) -> glm::Mat4 {
        let config = self.config_manager.get();
        let aspect_ratio = (screen_size.x as f32) / (screen_size.y as f32);
        glm::ext::perspective(config.zoom,aspect_ratio, config.z_near, config.z_far)
    }

    pub fn view(&self) -> glm::Mat4 {
        let config = self.config_manager.get();
        let position = glm::vec3(config.position.0, config.position.1, config.position.2);
        let lookat = glm::builtin::normalize(glm::vec3(config.lookat.0, config.lookat.1, config.lookat.2));
        let right = glm::builtin::normalize(glm::vec3(config.right.0, config.right.1, config.right.2));
        let up = glm::builtin::cross(lookat, right);
        glm::ext::look_at(position, position + lookat, up)
    }

    pub fn update(&mut self) {
        if self.config_manager.update() {
            println!("Updated!");
        }
    }
}
