use controls::{
    ControlEvent::CameraMove,
    Controller,
};
use dimensions::{
    Direction,
    time::DeltaTime
};
use glm::{
    Mat4,
    self,
    Vec3
};

pub struct Camera {
    position: Vec3,
    lookat: Vec3,
    up: Vec3,
}

impl Camera {
    pub fn new() -> Camera {
        let lookat = glm::vec3(0.0, 0.0, -1.0);
        let up = glm::vec3(0.0, 1.0, 0.0);
        Camera {
            position: glm::vec3(0.0, 0.0, 0.0),
            lookat,
            up,
        }
    }

    pub fn position(&self) -> Vec3 {
        self.position
    }

    pub fn view(&self) -> Mat4 {
        glm::ext::look_at(self.position, self.position + self.lookat, self.up)
    }

    pub fn update(&mut self, controller: &Controller, dt: DeltaTime) {
        let cam_speed : f32 = 8e-6;
        let movement = (dt.as_microseconds() as f32) * cam_speed;
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
