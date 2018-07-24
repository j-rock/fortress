use app::StatusOr;
use controls::{
    ControlEvent::CameraMove,
    Controller,
};
use dimensions::{
    Direction,
    time::DeltaTime
};
use file::{
    ConfigWatcher,
    SimpleConfigManager,
};
use glm::{
    Mat4,
    self,
    Vec3
};

#[derive(Deserialize)]
struct CameraConfig {
    left: f32,
    right: f32,
    bottom: f32,
    top: f32,
    z_near: f32,
    z_far: f32,
    cam_speed: f32
}

pub struct Camera {
    config_manager: SimpleConfigManager<CameraConfig>,
    position: Vec3,
}

impl Camera {
    pub fn new(config_watcher: &mut ConfigWatcher) -> StatusOr<Camera> {
        let config_manager = SimpleConfigManager::new(config_watcher, "camera.conf")?;
        Ok(Camera {
            config_manager,
            position: glm::vec3(32.5, 9.0, 10.0),
        })
    }

    pub fn position(&self) -> Vec3 {
        self.position
    }

    pub fn projection(&self) -> Mat4 {
        let config = self.config_manager.get();
        Self::ortho(config.left, config.right, config.bottom, config.top, config.z_near, config.z_far)
    }

    fn ortho(left: f32, right: f32, bottom: f32, top: f32, z_near: f32, z_far: f32) -> Mat4 {
        let rml = right - left;
        let tmb = top - bottom;
        let fmn = z_far - z_near;

        glm::Matrix4::new(
            glm::vec4(2.0 / rml, 0.0, 0.0, 0.0),
            glm::vec4(0.0, 2.0 / tmb, 0.0, 0.0),
            glm::vec4(0.0, 0.0, -2.0 / fmn, 0.0),
            glm::vec4(-(right + left) / rml, -(top + bottom) / tmb, -(z_far + z_near) / fmn, 0.0))
    }

    pub fn view(&self) -> Mat4 {
        glm::ext::look_at(self.position, self.position + glm::vec3(0.0, 0.0, -1.0), glm::vec3(0.0, 1.0, 0.0))
    }

    pub fn update(&mut self, controller: &Controller, dt: DeltaTime) {
        self.config_manager.update();

        let movement = (dt.as_microseconds() as f32) * self.config_manager.get().cam_speed;
        if controller.is_pressed(CameraMove(Direction::Up)) {
            self.position.y += movement;
        }
        if controller.is_pressed(CameraMove(Direction::Down)) {
            self.position.y -= movement;
        }
        if controller.is_pressed(CameraMove(Direction::Left)) {
            self.position.x -= movement;
        }
        if controller.is_pressed(CameraMove(Direction::Right)) {
            self.position.x += movement;
        }
        if controller.is_pressed(CameraMove(Direction::Backward)) {
            self.position.z += movement;
        }
        if controller.is_pressed(CameraMove(Direction::Forward)) {
            self.position.z -= movement;
        }
    }
}
