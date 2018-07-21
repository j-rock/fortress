use controls::Controller;
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

    pub fn update(&mut self, controller: &Controller, dt: DeltaTime) {
        self.camera.update(controller, dt);
    }

    pub fn draw_geometry(&self) {
        // let projection_view = self.camera.projection(canvas) * self.camera.view();
    }
}