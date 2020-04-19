use crate::{
    app::StatusOr,
    file::{
        self,
        ConfigWatcher,
        SimpleConfigManager,
    },
    render::{
        BitmapTexture,
        CameraGeometry,
        TextureUnit,
    },
    text::{
        PackedGlyphSheet,
        ScreenTextRenderer,
        ScreenTextRequest,
        TextConfig,
        TextContent,
        TextResolver,
        WorldTextRenderer,
        WorldTextRequest,
    },
};
use glm;

pub struct TextRenderer {
    config: SimpleConfigManager<TextConfig>,
    texture: BitmapTexture,
    resolver: TextResolver,
    screen_renderer: ScreenTextRenderer,
    world_renderer: WorldTextRenderer,
}

impl TextRenderer {
    pub fn new(config_watcher: &mut ConfigWatcher) -> StatusOr<Self> {
        let config = SimpleConfigManager::from_config_resource(config_watcher, "text.conf")?;

        let (texture, resolver) = {
            let config = config.get();
            let fonts = file::util::resource_base().join("fonts");
            let packed = PackedGlyphSheet::new(config, &fonts)?;
            let texture = BitmapTexture::new(packed.image, config.texture_atlas_style, TextureUnit::Texture0);
            let resolver = TextResolver::new(config, packed.mappings);
            (texture, resolver)
        };

        let screen_renderer = ScreenTextRenderer::new()?;
        let world_renderer = WorldTextRenderer::new()?;

        Ok(TextRenderer {
            config,
            texture,
            resolver,
            screen_renderer,
            world_renderer,
        })
    }

    pub fn pre_update(&mut self) {
        if self.config.update() {
            let config = self.config.get();
            let fonts = file::util::resource_base().join("fonts");
            match PackedGlyphSheet::new(config, &fonts) {
                Err(e) => println!("Couldn't reload text glyphs: {:?}", e),
                Ok(packed) => {
                    self.texture = BitmapTexture::new(packed.image, config.texture_atlas_style, TextureUnit::Texture0);
                    self.resolver = TextResolver::new(config, packed.mappings);
                },
            }
        }
    }

    pub fn update_render_info(&mut self, camera_geometry: &CameraGeometry, screen_size: glm::IVec2) {
        self.screen_renderer.set_screen_size(screen_size);
        self.world_renderer.set_parameters(self.config.get(), camera_geometry);
    }

    pub fn queue_screen_text(&mut self, content: impl Iterator<Item=TextContent>, request: ScreenTextRequest) {
        let current_locale = self.config.get().current_locale;
        self.screen_renderer.queue(&self.resolver, current_locale, content, request);
    }

    pub fn queue_world_text(&mut self, content: impl Iterator<Item=TextContent>, request: WorldTextRequest) {
        let current_locale = self.config.get().current_locale;
        self.world_renderer.queue(&self.resolver, current_locale, content, request);
    }

    pub fn draw(&mut self, camera_geometry: &CameraGeometry) {
        self.screen_renderer.draw(&self.texture);
        self.world_renderer.draw(&self.texture, camera_geometry);
    }
}
