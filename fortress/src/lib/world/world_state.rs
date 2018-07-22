use app::StatusOr;
use controls::Controller;
use dimensions::time::DeltaTime;
use file::ConfigWatcher;
use world::{
    Camera,
    Map,
    PhysicsSimulation,
};

pub struct WorldState {
    camera: Camera,
    physics_sim: PhysicsSimulation,
    map: Map,
}

impl WorldState {
    pub fn new(config_watcher: &mut ConfigWatcher) -> StatusOr<WorldState> {
        let mut physics_sim = PhysicsSimulation::new(config_watcher)?;
        let map = Map::new(config_watcher, &mut physics_sim)?;
        Ok(WorldState {
            camera: Camera::new(),
            physics_sim,
            map
        })
    }

    pub fn update(&mut self, controller: &Controller, dt: DeltaTime) {
        self.camera.update(controller, dt);
        self.map.update();
        self.physics_sim.update(dt);
    }

    pub fn draw_geometry(&self) {
        // let projection_view = self.camera.ortho() * self.camera.view();
    }
}