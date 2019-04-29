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
        BackgroundRenderer,
        Camera,
        FullyIlluminatedSpriteRenderer,
        HexRenderer,
        LightDependentSpriteRenderer,
        PointLight,
        SpriteSheetTextureManager,
        Viewport,
    },
    treasures::TreasureSystem,
    weapons::WeaponMatchers,
    world::{
        RandGen,
        WorldView
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
    rng: RandGen,

    textures: SpriteSheetTextureManager,
    background_renderer: Option<BackgroundRenderer>,
    hex_renderer: HexRenderer,
    full_light_sprite: FullyIlluminatedSpriteRenderer,
    light_dependent_sprite: LightDependentSpriteRenderer,
    lights: Vec<PointLight>,

    map: Map,
    players: PlayerSystem,
    treasures: TreasureSystem,

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

        let mut rng = RandGen::new();

        let map = Map::new(config_watcher, &mut physics_sim)?;
        let players = PlayerSystem::new(config_watcher, map.spawns())?;
        let treasures = TreasureSystem::new(config_watcher, map.treasure_chests(), &mut rng, &mut physics_sim)?;

        Ok(WorldState {
            config_manager: SimpleConfigManager::from_config_resource(config_watcher, "world.conf")?,
            camera: Camera::new(config_watcher)?,
            rng,
            textures: SpriteSheetTextureManager::new(config_watcher)?,
            background_renderer: None,
            hex_renderer: HexRenderer::new()?,
            full_light_sprite: FullyIlluminatedSpriteRenderer::new()?,
            light_dependent_sprite: LightDependentSpriteRenderer::new()?,
            lights: vec!(),
            map,
            players,
            treasures,
            physics_sim
        })
    }

    pub fn update(&mut self, audio: &AudioPlayer, controller: &Controller, dt: DeltaTime) {
        self.config_manager.update();
        self.camera.update();
        self.textures.update();

        // Pre-update.
        {
            if self.map.pre_update(&mut self.physics_sim) {
                self.players.respawn(self.map.spawns());
                self.treasures.respawn(self.map.treasure_chests(), &mut self.rng, &mut self.physics_sim);
            } else {
                self.players.pre_update(audio, controller, &mut self.physics_sim, dt);
                self.treasures.pre_update(controller, &mut self.rng, &mut self.physics_sim);
            }
        }

        {
            let world_view = WorldView {
                audio,
                players: &mut self.players,
                treasures: &mut self.treasures,
                dt
            };
            self.physics_sim.borrow_mut().step(world_view);
        }

        // Post-update.
        {
            self.players.post_update(audio);
            self.treasures.post_update(&mut self.physics_sim);
        }
    }

    pub fn clear_color(&self) -> (f32, f32, f32) {
       self.config_manager.get().clear_color
    }

    pub fn draw(&mut self, screen_size: glm::IVec2) {
        self.lights.clear();
        self.populate_lights();
        self.draw_geometry(screen_size);
    }

    fn populate_lights(&mut self) {
        self.map.populate_lights(&mut self.lights);
        self.players.populate_lights(&mut self.lights);
        self.treasures.populate_lights(&mut self.lights);
    }

    fn draw_geometry(&mut self, screen_size: glm::IVec2) {
        let (lookat, right, up) = self.camera.lookat_right_and_up();
        let projection_view = self.camera.projection(screen_size) * self.camera.view(lookat, up);

        self.map.queue_draw(&mut self.hex_renderer, &mut self.full_light_sprite);
        self.players.queue_draw(&mut self.full_light_sprite, &mut self.light_dependent_sprite);
        self.treasures.queue_draw(&mut self.full_light_sprite, &mut self.light_dependent_sprite);

        if let Some(background_renderer) = self.background_renderer.as_mut() {
            background_renderer.draw();
        }
        self.full_light_sprite.draw(&self.textures, &projection_view, right, up);
        self.light_dependent_sprite.draw(&self.lights, &self.textures, &projection_view, right, up);
        self.hex_renderer.draw(&self.lights, &projection_view);

        // Fix viewport at the end.
        Viewport::default(screen_size).set();
    }
}
