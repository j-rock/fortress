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
        PointLight,
        SpriteRenderer,
        Viewport,
    },
    weapons::WeaponMatchers,
    world::WorldView,
};
use glm;

#[derive(Deserialize)]
struct WorldConfig {
    clear_color: (f32, f32, f32)
}

pub struct WorldState {
    config_manager: SimpleConfigManager<WorldConfig>,
    camera: Camera,
    sprite_renderer: SpriteRenderer,

    map: Map,
    players: PlayerSystem,

    // Declare physics simulation last so it is dropped last.
    physics_sim: PhysicsSimulation,
}

impl WorldState {
    pub fn new(config_watcher: &mut ConfigWatcher) -> StatusOr<WorldState> {
        let mut physics_sim = PhysicsSimulation::new(config_watcher)?;

        physics_sim.borrow_mut().add_contact_matchers(vec!(
            WeaponMatchers::bullet_hit(),
        ));
        physics_sim.borrow_mut().add_proximity_matchers(vec!(
        ));

        let map = Map::new(config_watcher, &mut physics_sim)?;
        let players = PlayerSystem::new(config_watcher, map.spawns())?;

        Ok(WorldState {
            config_manager: SimpleConfigManager::from_config_resource(config_watcher, "world.conf")?,
            camera: Camera::new(config_watcher)?,
            sprite_renderer: SpriteRenderer::new()?,
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

        {
            let world_view = WorldView {
                audio,
                players: &mut self.players,
                dt
            };
            self.physics_sim.borrow_mut().step(world_view);
        }

        // Post-update.
        {
            self.players.post_update(audio);
        }
    }

    pub fn clear_color(&self) -> (f32, f32, f32) {
       self.config_manager.get().clear_color
    }

    pub fn draw_geometry(&mut self, screen_size: glm::IVec2, lights: &mut Vec<PointLight>) {
        lights.push(PointLight {
            position: glm::vec3(32.5, 9.0, 0.0),
            color: glm::vec3(0.2, 0.2, 0.2),
            attenuation: glm::vec3(1.0, 0.005, 0.0004),
        });

        let (lookat, right, up) = self.camera.lookat_right_and_up();
        let projection_view = self.camera.projection(screen_size) * self.camera.view(lookat, up);

        self.map.draw(&projection_view);
        self.players.draw(&mut self.sprite_renderer, lights);

        self.sprite_renderer.draw(&projection_view, right, up);

        // Fix viewport at the end.
        Viewport::default(screen_size).set();
    }
}
