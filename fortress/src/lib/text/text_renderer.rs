use crate::{
    app::StatusOr,
    file::{
        self,
        ConfigWatcher,
        SimpleConfigManager,
    },
    render::{
        BitmapTexture,
        TextureUnit,
    },
    text::{
        PackedGlyphSheet,
        ScreenTextRenderer,
        TextConfig,
        TextContent,
        TextRenderRequest,
        TextResolver,
    },
};
use glm;

pub struct TextRenderer {
    config: SimpleConfigManager<TextConfig>,
    texture: BitmapTexture,
    resolver: TextResolver,
    screen_renderer: ScreenTextRenderer,
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

        Ok(TextRenderer {
            config,
            texture,
            resolver,
            screen_renderer,
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

    pub fn set_screen_size(&mut self, screen_size: glm::IVec2) {
        self.screen_renderer.set_screen_size(screen_size);
    }

    pub fn queue(&mut self, content: impl Iterator<Item=TextContent>, request: TextRenderRequest) {
        let current_locale = self.config.get().current_locale;
        self.screen_renderer.queue(&self.resolver, current_locale, content, request);
    }

    pub fn draw(&mut self) {
        self.screen_renderer.draw(&self.texture);
    }
}
