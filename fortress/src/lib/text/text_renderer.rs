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
        Base10CharIterator,
        GlyphId,
        GlyphInfo,
        Locale,
        NamedText,
        PackedGlyphSheet,
        ScreenTextRenderer,
        TextConfig,
        TextContent,
        TextRenderRequest,
    },
};
use glm;
use std::collections::HashMap;

pub struct TextRenderer {
    config: SimpleConfigManager<TextConfig>,
    localized_text: HashMap<(Locale, NamedText), String>,
    texture: BitmapTexture,
    mappings: HashMap<GlyphId, GlyphInfo>,
    screen_renderer: ScreenTextRenderer,
}

impl TextRenderer {
    pub fn new(config_watcher: &mut ConfigWatcher) -> StatusOr<Self> {
        let config = SimpleConfigManager::from_config_resource(config_watcher, "text.conf")?;

        let (localized_text, texture, mappings) = {
            let config = config.get();
            let localized_text = Self::compute_all_text(config);
            let fonts = file::util::resource_base().join("fonts");
            let packed = PackedGlyphSheet::new(config, &fonts)?;
            let texture = BitmapTexture::new(packed.image, config.texture_atlas_style, TextureUnit::Texture0);
            (localized_text, texture, packed.mappings)
        };

        let screen_renderer = ScreenTextRenderer::new()?;

        Ok(TextRenderer {
            config,
            localized_text,
            texture,
            mappings,
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
                    self.localized_text = Self::compute_all_text(config);
                    self.texture = BitmapTexture::new(packed.image, config.texture_atlas_style, TextureUnit::Texture0);
                    self.mappings = packed.mappings;
                },
            }
        }
    }

    pub fn queue(&mut self, request: TextRenderRequest) {
        match request.content {
            TextContent::Number(number) => {
                if let Some(char_iterator) = Base10CharIterator::new(number) {
                    self.screen_renderer.queue(&self.mappings, &request, char_iterator);
                }
            },
            TextContent::Text(text) => {
                let current_locale = self.config.get().current_locale;
                if let Some(text) = self.localized_text.get(&(current_locale, text)) {
                    self.screen_renderer.queue(&self.mappings, &request, text.chars());
                }
            },
        }
    }

    pub fn draw(&mut self, screen_size: glm::IVec2) {
        self.screen_renderer.draw(screen_size, &self.texture);
    }

    fn compute_all_text(config: &TextConfig) -> HashMap<(Locale, NamedText), String> {
        config.localized_text
            .iter()
            .flat_map(|(locale, named_text_map)| {
                let locale = *locale;
                named_text_map
                    .iter()
                    .map(move |(named_text, text)| {
                        ((locale, *named_text), text.clone())
                    })
            })
            .collect()
    }
}
