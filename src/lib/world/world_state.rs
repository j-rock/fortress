use crate::{
    app::StatusOr,
    audio::AudioPlayer,
    control::Controller,
    dimensions::time::DeltaTime,
    file::{
        ConfigWatcher,
        SimpleConfigManager,
    },
    maps::Map,
    physics::PhysicsSimulation,
    players::PlayerSystem,
    render::{
        Camera,
        Viewport,
    },
};
use glm;

#[derive(Deserialize)]
struct WorldConfig {
    clear_color: (f32, f32, f32)
}

pub struct WorldState {
    config_manager: SimpleConfigManager<WorldConfig>,
    camera: Camera,

    map: Map,
    players: PlayerSystem,

    // Declare physics simulation last so it is dropped last.
    physics_sim: PhysicsSimulation,
}

impl WorldState {
    pub fn new(config_watcher: &mut ConfigWatcher) -> StatusOr<WorldState> {
        let mut physics_sim = PhysicsSimulation::new(config_watcher)?;

        physics_sim.borrow_mut().add_contact_matchers(vec!(
        ));
        physics_sim.borrow_mut().add_proximity_matchers(vec!(
        ));

        let map = Map::new(config_watcher, &mut physics_sim)?;
        let players = PlayerSystem::new(config_watcher, map.spawns())?;

        Ok(WorldState {
            config_manager: SimpleConfigManager::from_config_resource(config_watcher, "world.conf")?,
            camera: Camera::new(config_watcher)?,
            map,
            players,
            physics_sim
        })
    }

    pub fn update(&mut self, audio: &AudioPlayer, controller: &Controller, dt: DeltaTime) {
        self.config_manager.update();
        self.camera.update();

        // Pre-update.
        {
            if self.map.pre_update(&mut self.physics_sim) {
                self.players.respawn(self.map.spawns());
            } else {
                self.players.pre_update(audio, controller, &mut self.physics_sim, dt);
            }
        }

        self.physics_sim.borrow_mut().step(audio, dt);

        // Post-update.
        {
            self.players.post_update(audio);
        }
    }

    pub fn clear_color(&self) -> (f32, f32, f32) {
       self.config_manager.get().clear_color
    }

    pub fn draw_geometry(&mut self, screen_size: glm::IVec2) {
        let projection_view = self.camera.projection(screen_size) * self.camera.view();

        self.map.draw(&projection_view);
        self.players.draw();

        // Fix viewport at the end.
        Viewport::default(screen_size).set();
    }
}