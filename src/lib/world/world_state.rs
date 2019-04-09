use crate::{
    app::StatusOr,
    audio::AudioPlayer,
    control::Controller,
    dimensions::time::DeltaTime,
    file::{
        ConfigWatcher,
        SimpleConfigManager,
    },
    physics::PhysicsSimulation,
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
    // Declare physics simulation last so it is dropped last.
    physics_sim: PhysicsSimulation,
}

impl WorldState {
    pub fn new(config_watcher: &mut ConfigWatcher) -> StatusOr<WorldState> {
        let physics_sim = PhysicsSimulation::new(config_watcher)?;

        physics_sim.borrow_mut().add_contact_matchers(vec!(
        ));
        physics_sim.borrow_mut().add_proximity_matchers(vec!(
        ));

        Ok(WorldState {
            config_manager: SimpleConfigManager::from_config_resource(config_watcher, "world.conf")?,
            camera: Camera::new(config_watcher)?,
            physics_sim
        })
    }

    pub fn register(&mut self) {
    }

    pub fn update(&mut self, audio: &AudioPlayer, _controller: &Controller, dt: DeltaTime) {
        self.config_manager.update();
        self.camera.update();

        // Pre-update.
        {
        }

        self.physics_sim.borrow_mut().update(audio, dt);

        // Post-update.
        {
        }
    }

    pub fn clear_color(&self) -> (f32, f32, f32) {
       self.config_manager.get().clear_color
    }

    pub fn draw_geometry(&mut self, screen_size: glm::IVec2) {
        Viewport::default(screen_size).set();
    }
}