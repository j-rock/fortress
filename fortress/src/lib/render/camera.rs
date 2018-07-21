use controls::KeyboardControls;
use dimensions::time::DeltaTime;
use glm::{
    Mat4,
    self,
    Vec3
};
use sdl2::{
    keyboard::Scancode,
    render::WindowCanvas,
};

pub struct Camera {
    position: Vec3,
    lookat: Vec3,
    up: Vec3,
    zoom: f32,
}

impl Camera {
    pub fn new() -> Camera {
        let lookat = glm::builtin::normalize(glm::vec3(0.0, -0.267, -0.96));
        let up = glm::vec3(0.0, -lookat.z, lookat.y);
        Camera {
            position: glm::vec3(0.9685, 34.0755, 164.9),
            lookat,
            up,
            zoom: 45.0,
        }
    }

    pub fn position(&self) -> Vec3 {
        self.position
    }

    pub fn projection(&self, canvas: &WindowCanvas) -> Mat4 {
        let (width, height) = canvas.window().size();
        let (float_width, float_height) = (width as f32, height as f32);
        let (z_near, z_far) = (0.1, 1000.0);
        glm::ext::perspective(self.zoom, float_width / float_height, z_near, z_far)
    }

    pub fn view(&self) -> Mat4 {
        glm::ext::look_at(self.position, self.position + self.lookat, self.up)
    }

    pub fn update(&mut self, keyboard: &KeyboardControls, dt: DeltaTime) {
        if keyboard.just_pressed(Scancode::M) {
            println!("Cam Position {:?}, Lookat {:?}, Up {:?}", self.position, self.lookat, self.up);
        }
        let cam_speed : f32 = 8e-6;
        let movement = (dt.as_microseconds() as f32) * cam_speed;
        if keyboard.is_pressed(Scancode::D) {
            self.position.x += movement;
        }
        if keyboard.is_pressed(Scancode::A) {
            self.position.x -= movement;
        }
        if keyboard.is_pressed(Scancode::W) {
            self.position.y += movement;
        }
        if keyboard.is_pressed(Scancode::S) {
            self.position.y -= movement;
        }
        if keyboard.is_pressed(Scancode::F) {
            self.position.z += movement;
        }
        if keyboard.is_pressed(Scancode::R) {
            self.position.z -= movement;
        }
        if keyboard.is_pressed(Scancode::I) {
            let right = glm::vec3(1.0, 0.0, 0.0);
            let eye = glm::mat4(
                1.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 1.0
             );
            let rotation_x = glm::ext::rotate(&eye, movement / 20.0, right);
            let lookat4 = rotation_x * glm::vec4(self.lookat.x, self.lookat.y, self.lookat.z, 1.0);
            self.lookat = glm::ext::normalize_to(glm::vec3(lookat4.x, lookat4.y, lookat4.z), 1.0);
            self.up = glm::builtin::cross(right, self.lookat);
        }
        if keyboard.is_pressed(Scancode::K) {
            let right = glm::vec3(1.0, 0.0, 0.0);
            let eye = glm::mat4(
                1.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 1.0
            );
            let rotation_x = glm::ext::rotate(&eye, -movement / 20.0, right);
            let lookat4 = rotation_x * glm::vec4(self.lookat.x, self.lookat.y, self.lookat.z, 1.0);
            self.lookat = glm::ext::normalize_to(glm::vec3(lookat4.x, lookat4.y, lookat4.z), 1.0);
            self.up = glm::builtin::cross(right, self.lookat);
        }
    }
}
