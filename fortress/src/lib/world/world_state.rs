use crate::{
    app::{
        RandGen,
        StatusOr
    },
    audio::AudioPlayer,
    control::Controller,
    dimensions::time::DeltaTime,
    enemies::EnemySystem,
    file::{
        ConfigWatcher,
        SimpleConfigManager,
    },
    items::ItemSystem,
    maps::Map,
    particles::ParticleSystem,
    physics::PhysicsSimulation,
    players::{
        PlayerMatchers,
        PlayerSystem,
    },
    render::{
        BackgroundRenderer,
        Camera,
        CameraStreamInfo,
        FullyIlluminatedSpriteRenderer,
        HexRenderer,
        LightDependentSpriteRenderer,
        PointLights,
        SpriteSheetTextureManager,
        Viewport,
    },
    text::TextRenderer,
    weapons::WeaponMatchers,
    world::{
        WorldUi,
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
    world_ui: WorldUi,

    textures: SpriteSheetTextureManager,
    text_renderer: TextRenderer,
    background_renderer: BackgroundRenderer,
    hex_renderer: HexRenderer,
    full_light_sprite: FullyIlluminatedSpriteRenderer,
    light_dependent_sprite: LightDependentSpriteRenderer,
    lights: PointLights,

    map: Map,
    players: PlayerSystem,
    enemies: EnemySystem,
    items: ItemSystem,
    particles: ParticleSystem,

    // Declare physics simulation last so it is dropped last.
    physics_sim: PhysicsSimulation,
}

impl WorldState {
    pub fn new(config_watcher: &mut ConfigWatcher) -> StatusOr<WorldState> {
        let mut physics_sim = PhysicsSimulation::new(config_watcher)?;

        physics_sim.borrow_mut().add_contact_matchers(vec!(
        ));
        physics_sim.borrow_mut().add_proximity_matchers(vec!(
            WeaponMatchers::bullet_hit_proximity_matcher(),
            PlayerMatchers::player_collected_item(),
        ));

        let map = Map::new(config_watcher, &mut physics_sim)?;
        let players = PlayerSystem::new(config_watcher, map.spawns())?;
        let enemies = EnemySystem::new(config_watcher, map.enemy_generators(), &mut physics_sim)?;
        let items = ItemSystem::new(config_watcher)?;
        let particles = ParticleSystem::new(config_watcher)?;
        let lights = PointLights::new()?;

        Ok(WorldState {
            config_manager: SimpleConfigManager::from_config_resource(config_watcher, "world.conf")?,
            camera: Camera::new(config_watcher)?,
            world_ui: WorldUi::new(config_watcher)?,
            textures: SpriteSheetTextureManager::new(config_watcher)?,
            text_renderer: TextRenderer::new(config_watcher)?,
            background_renderer: BackgroundRenderer::new(config_watcher)?,
            hex_renderer: HexRenderer::new()?,
            full_light_sprite: FullyIlluminatedSpriteRenderer::new()?,
            light_dependent_sprite: LightDependentSpriteRenderer::new()?,
            lights,
            map,
            players,
            enemies,
            items,
            particles,
            physics_sim
        })
    }

    pub fn update(&mut self, audio: &AudioPlayer, controller: &Controller, rng: &mut RandGen, dt: DeltaTime) {
        self.config_manager.update();
        self.textures.update();

        // Pre-update.
        {
            self.background_renderer.pre_update();
            self.text_renderer.pre_update();
            self.camera.pre_update(dt);
            self.world_ui.pre_update(dt);

            if self.map.pre_update(&mut self.physics_sim) {
                self.players.respawn(self.map.spawns());
                self.enemies.respawn(self.map.enemy_generators(), &mut self.physics_sim);
                self.items.respawn();
                self.particles.respawn();
            } else {
                self.players.pre_update(audio, controller, &mut self.particles, rng, self.camera.mut_shake(), &mut self.physics_sim, dt);
                let player_locs = self.players.player_locs();
                self.enemies.pre_update(controller, dt, player_locs, &mut self.physics_sim);
                self.items.pre_update();
                self.particles.pre_update(dt);
            }
        }

        {
            self.physics_sim.borrow_mut().step(dt);
            self.physics_sim.borrow().process_contacts(WorldView {
                audio,
                players: &mut self.players,
                enemies: &mut self.enemies,
                items: &mut self.items,
                particles: &mut self.particles,
                dt
            });
        }

        // Post-update.
        {
            self.players.post_update();
            self.camera.post_update(self.players.player_locs(), dt);
            self.items.post_update();
            self.enemies.post_update(audio, &mut self.items, self.camera.mut_shake(), &mut self.physics_sim);

            let camera_stream_info = self.camera.stream_info(self.map.hex_cell_length());
            self.particles.post_update(&camera_stream_info, rng);
        }
    }

    pub fn clear_color(&self) -> (f32, f32, f32) {
       self.config_manager.get().clear_color
    }

    pub fn draw(&mut self, screen_size: glm::IVec2) {
        let camera_stream_info = self.camera.stream_info(self.map.hex_cell_length());

        self.populate_lights(&camera_stream_info);
        self.draw_geometry(&camera_stream_info, screen_size);
        self.lights.clear();
    }

    fn populate_lights(&mut self, camera_stream_info: &CameraStreamInfo) {
        self.lights.set_camera_stream_info(camera_stream_info.clone());
        self.map.populate_lights(&mut self.lights);
        self.players.populate_lights(&mut self.lights);
        self.enemies.populate_lights(&mut self.lights);
    }

    fn draw_geometry(&mut self, camera_stream_info: &CameraStreamInfo, screen_size: glm::IVec2) {
        let geometry = self.camera.geometry(screen_size);

        self.text_renderer.set_screen_size(screen_size);
        self.light_dependent_sprite.set_camera_stream_info(camera_stream_info.clone());

        self.world_ui.queue_draw(&mut self.text_renderer);
        self.map.queue_draw(&camera_stream_info, &mut self.hex_renderer, &mut self.full_light_sprite);
        self.players.queue_draw(&mut self.full_light_sprite, &mut self.light_dependent_sprite);
        self.enemies.queue_draw(&mut self.light_dependent_sprite);
        self.items.queue_draw(&mut self.light_dependent_sprite);

        self.background_renderer.draw(&self.textures, &geometry);
        self.full_light_sprite.draw(&self.textures, &geometry);
        self.light_dependent_sprite.draw(&self.lights, &self.textures, &geometry);
        self.hex_renderer.draw(&self.textures, &self.lights, &geometry);
        // Draw particles after hex ground to not mess up transparency.
        self.particles.draw(&camera_stream_info, &geometry);
        self.text_renderer.draw();

        // Fix viewport at the end.
        Viewport::default(screen_size).set();
    }
}
