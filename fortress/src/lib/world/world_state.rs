use app::StatusOr;
use control::Controller;
use dimensions::time::DeltaTime;
use file::{
    ConfigWatcher,
    SimpleConfigManager,
};
use map::Map;
use physics::PhysicsSimulation;
use player::Player;
use render::BoxRenderer;
use weapon::Crossbow;
use world::Camera;
use wraith::Wraith;

#[derive(Deserialize)]
struct WorldConfig {
    clear_color: (f32, f32, f32)
}

pub struct WorldState {
    config_manager: SimpleConfigManager<WorldConfig>,
    camera: Camera,
    map: Map,
    player: Player,
    wraith: Wraith,

    box_renderer: BoxRenderer,
    // Declare physics simulation last so it is dropped last.
    physics_sim: PhysicsSimulation,
}

impl WorldState {
    pub fn new(config_watcher: &mut ConfigWatcher) -> StatusOr<WorldState> {
        let mut physics_sim = PhysicsSimulation::new(config_watcher)?;
        let map = Map::new(config_watcher, &mut physics_sim)?;
        let player = Player::new(config_watcher, &mut physics_sim)?;
        let wraith = Wraith::new(config_watcher, &mut physics_sim)?;

        physics_sim.add_collision_matchers(vec!(
            Player::foot_sensor_hit_something(),
            Player::slash_wraith(),
            Crossbow::arrow_hit(),
        ));

        Ok(WorldState {
            config_manager: SimpleConfigManager::new(config_watcher, "world.conf")?,
            camera: Camera::new(config_watcher)?,
            map,
            player,
            wraith,
            box_renderer: BoxRenderer::new()?,
            physics_sim
        })
    }

    pub fn register(&mut self) {
        self.map.register();
        self.player.register();
        self.wraith.register();
    }

    pub fn update(&mut self, controller: &Controller, dt: DeltaTime) {
        self.config_manager.update();
        self.camera.update();

        {
            self.map.pre_update(controller, dt);
            self.player.pre_update(controller, dt);
            self.wraith.pre_update(controller, dt);
        }

        self.physics_sim.update(dt);

        {
            self.player.post_update();
            self.wraith.post_update();
        }
    }

    pub fn clear_color(&self) -> (f32, f32, f32) {
       self.config_manager.get().clear_color
    }

    pub fn draw_geometry(&mut self) {
        self.map.draw(&mut self.box_renderer);
        self.player.draw(&mut self.box_renderer);
        self.wraith.draw(&mut self.box_renderer);

        let projection_view = self.camera.projection() * self.camera.view();
        self.box_renderer.draw(&projection_view);
    }
}