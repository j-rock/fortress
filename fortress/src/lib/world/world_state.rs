use app::StatusOr;
use controls::Controller;
use dimensions::time::DeltaTime;
use file::{
    ConfigWatcher,
    SimpleConfigManager,
};
use map::Map;
use physics::PhysicsSimulation;
use player::Player;
use world::Camera;

#[derive(Deserialize)]
struct WorldConfig {
    clear_color: (f32, f32, f32)
}

pub struct WorldState {
    config_manager: SimpleConfigManager<WorldConfig>,
    camera: Camera,
    physics_sim: PhysicsSimulation,
    map: Map,
    player: Player,
}

impl WorldState {
    pub fn new(config_watcher: &mut ConfigWatcher) -> StatusOr<WorldState> {
        let mut physics_sim = PhysicsSimulation::new(config_watcher)?;
        let map = Map::new(config_watcher, &mut physics_sim)?;
        let player = Player::new(config_watcher, &mut physics_sim)?;
        Ok(WorldState {
            config_manager: SimpleConfigManager::new(config_watcher, "world.conf")?,
            camera: Camera::new(config_watcher)?,
            physics_sim,
            map,
            player
        })
    }

    pub fn update(&mut self, controller: &Controller, dt: DeltaTime) {
        self.config_manager.update();
        self.camera.update();

        {
            let registrar = self.physics_sim.registrar_mut();
            self.map.update(registrar);
            self.player.update(registrar, controller, dt);
        }

        // Physics simulation must update last.
        self.physics_sim.update(dt);
    }

    pub fn clear_color(&self) -> (f32, f32, f32) {
       self.config_manager.get().clear_color
    }

    pub fn draw_geometry(&mut self) {
        let projection_view = self.camera.projection() * self.camera.view();
        self.map.draw(&projection_view);
        self.player.draw(&projection_view);
    }
}