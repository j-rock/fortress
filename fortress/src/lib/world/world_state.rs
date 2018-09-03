use app::StatusOr;
use camera::Camera;
use control::Controller;
use dimensions::time::DeltaTime;
use file::{
    ConfigWatcher,
    SimpleConfigManager,
};
use map::Map;
use physics::PhysicsSimulation;
use player::{
    Player,
    PlayerSystem
};
use render::BoxRenderer;
use weapon::Crossbow;
use wraith::Wraith;

#[derive(Deserialize)]
struct WorldConfig {
    clear_color: (f32, f32, f32)
}

pub struct WorldState {
    config_manager: SimpleConfigManager<WorldConfig>,
    camera: Camera,
    map: Map,
    players: PlayerSystem,
    wraith: Wraith,

    box_renderer: BoxRenderer,
    // Declare physics simulation last so it is dropped last.
    physics_sim: PhysicsSimulation,
}

impl WorldState {
    pub fn new(config_watcher: &mut ConfigWatcher) -> StatusOr<WorldState> {
        let mut physics_sim = PhysicsSimulation::new(config_watcher)?;
        let map = Map::new(config_watcher, &mut physics_sim)?;
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
            players: PlayerSystem::new(config_watcher)?,
            wraith,
            box_renderer: BoxRenderer::new()?,
            physics_sim
        })
    }

    pub fn register(&mut self) {
        self.map.register();
        self.wraith.register();
    }

    pub fn update(&mut self, controller: &Controller, dt: DeltaTime) {
        self.config_manager.update();
        self.camera.update();

        {
            self.map.pre_update(controller, dt);
            self.players.pre_update(controller, &mut self.physics_sim, dt);
            self.wraith.pre_update(controller, dt);
        }

        self.physics_sim.update(dt);

        {
            self.players.post_update();
            self.wraith.post_update();
        }
    }

    pub fn clear_color(&self) -> (f32, f32, f32) {
       self.config_manager.get().clear_color
    }

    pub fn draw_geometry(&mut self) {
        self.map.draw(&mut self.box_renderer);
        self.players.draw(&mut self.box_renderer);
        self.wraith.draw(&mut self.box_renderer);

        let player1_pos = self.players.get_player1_pos();
        let projection_view = self.camera.projection() * self.camera.view(player1_pos);
        self.box_renderer.draw(&projection_view);
    }
}