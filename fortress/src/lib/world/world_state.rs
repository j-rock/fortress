use controls::KeyboardControls;
use dimensions::time::DeltaTime;
use world::Camera;

pub struct WorldState {
    camera: Camera,
}

impl WorldState {
    pub fn new() -> WorldState {
        WorldState {
            camera: Camera::new()
        }
    }

    pub fn update(&mut self, keyboard: &KeyboardControls, dt: DeltaTime) {
        self.camera.update(keyboard, dt);
    }

    pub fn draw_geometry(&self) {
        // let projection_view = self.camera.projection(canvas) * self.camera.view();
    }
}