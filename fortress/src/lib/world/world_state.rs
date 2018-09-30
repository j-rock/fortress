use app::StatusOr;
use audio::AudioPlayer;
use buff::{
    BuffBox,
    BuffSystem,
};
use control::Controller;
use dimensions::time::DeltaTime;
use file::{
    ConfigWatcher,
    SimpleConfigManager,
};
use glm;
use map::Map;
use physics::PhysicsSimulation;
use player::{
    Player,
    PlayerSystem
};
use render::{
    BoxRenderer,
    Camera,
    Viewport,
};
use weapon::Crossbow;
use world::RandGen;

#[derive(Deserialize)]
struct WorldConfig {
    clear_color: (f32, f32, f32)
}

pub struct WorldState {
    config_manager: SimpleConfigManager<WorldConfig>,
    camera: Camera,
    map: Map,
    players: PlayerSystem,
    buffs: BuffSystem,
    rng: RandGen,

    box_renderer: BoxRenderer,
    // Declare physics simulation last so it is dropped last.
    physics_sim: PhysicsSimulation,
}

impl WorldState {
    pub fn new(config_watcher: &mut ConfigWatcher) -> StatusOr<WorldState> {
        let mut physics_sim = PhysicsSimulation::new(config_watcher)?;
        let map = Map::new(config_watcher, &mut physics_sim)?;
        let mut rng = RandGen::new();
        let buffs = BuffSystem::new(config_watcher, map.get_buff_box_spawns(), &mut rng, &mut physics_sim)?;
        let players = PlayerSystem::new(config_watcher, map.get_player_spawns())?;

        physics_sim.add_collision_matchers(vec!(
            Player::foot_sensor_hit_something(),
            Crossbow::arrow_hit(),
            BuffBox::player_slashed_buff_box(),
            BuffBox::player_hit_buff_drop(),
        ));

        Ok(WorldState {
            config_manager: SimpleConfigManager::from_config_resource(config_watcher, "world.conf")?,
            camera: Camera::new(config_watcher)?,
            map,
            players,
            buffs,
            rng,
            box_renderer: BoxRenderer::new()?,
            physics_sim
        })
    }

    pub fn register(&mut self) {
        self.map.register();
    }

    pub fn update(&mut self, audio: &AudioPlayer, controller: &Controller, dt: DeltaTime) {
        self.config_manager.update();
        self.camera.update();

        {
            if self.map.pre_update(&mut self.physics_sim) {
                self.players.respawn(self.map.get_player_spawns());
                self.buffs.respawn(self.map.get_buff_box_spawns(), &mut self.rng, &mut self.physics_sim);
            } else {
                self.players.pre_update(audio, controller, &mut self.physics_sim, dt);
                self.buffs.pre_update(controller, &mut self.rng, &mut self.physics_sim);
            }
        }

        self.physics_sim.update(audio, dt);

        {
            self.players.post_update();
            self.buffs.post_update(&mut self.physics_sim);
        }
    }

    pub fn clear_color(&self) -> (f32, f32, f32) {
       self.config_manager.get().clear_color
    }

    pub fn draw_geometry(&mut self, screen_size: glm::IVec2) {
        self.map.draw(&mut self.box_renderer);
        self.players.draw(&mut self.box_renderer);
        self.buffs.draw(&mut self.box_renderer);

        self.box_renderer.draw_begin();
        {
            for camera_view in self.players.get_views(screen_size).into_iter() {
                camera_view.viewport.set();
                let projection_view = self.camera.projection(screen_size, camera_view.scale) * self.camera.view(camera_view.eye);
                self.box_renderer.draw(&projection_view);
            }
        }
        self.box_renderer.draw_end();

        Viewport::default(screen_size).set();
    }
}