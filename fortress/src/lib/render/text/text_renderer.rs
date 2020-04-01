use crate::{
    app::StatusOr,
    file::{
        ConfigWatcher,
        SimpleConfigManager,
    },
    render::{
        TextConfig,
        TextVertexInfo,
    },
};
use glyph_brush::{
    GlyphBrush,
    GlyphBrushBuilder,
};

pub struct TextRenderer {
    config: SimpleConfigManager<TextConfig>,
    glyph_brush: GlyphBrush<'static, TextVertexInfo>,
}

impl TextRenderer {
    pub fn new(config_watcher: &mut ConfigWatcher) -> StatusOr<Self> {
        let config = SimpleConfigManager::<TextConfig>::from_config_resource(config_watcher, "text.conf")?;
        let glyph_brush = Self::make_glyph_brush(config.get());

        Ok(TextRenderer {
            config,
            glyph_brush,
        })
    }

    pub fn pre_update(&mut self) {
        if self.config.update() {
            self.glyph_brush = Self::make_glyph_brush(self.config.get());
        }
    }

    fn make_glyph_brush(config: &TextConfig) -> GlyphBrush<'static, TextVertexInfo> {
        let font: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "\\fonts\\veger_regular.ttf"));
        GlyphBrushBuilder::using_font_bytes(font)
            .initial_cache_size(config.initial_cache_size)
            .build()
    }
}