use app::StatusOr;
use controls::Controller;
use dimensions::time::DeltaTime;
use file::ConfigWatcher;
use world::{
    Camera,
    Map,
};

pub struct WorldState {
    camera: Camera,
    map: Map,
}

impl WorldState {
    pub fn new(config_watcher: &mut ConfigWatcher) -> StatusOr<WorldState> {
        Ok(WorldState {
            camera: Camera::new(),
            map: Map::new(config_watcher)?
        })
    }

    pub fn update(&mut self, controller: &Controller, dt: DeltaTime) {
        self.camera.update(controller, dt);
        self.map.update();
    }

    pub fn draw_geometry(&self) {
        // let projection_view = self.camera.ortho() * self.camera.view();
    }
}