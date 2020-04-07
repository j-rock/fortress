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
        GlyphId,
        GlyphInfo,
        PackedGlyphSheet,
        TextConfig,
    },
};
use std::collections::HashMap;

pub struct TextRenderer {
    config: SimpleConfigManager<TextConfig>,
    texture: BitmapTexture,
    mappings: HashMap<GlyphId, GlyphInfo>,
}

impl TextRenderer {
    pub fn new(config_watcher: &mut ConfigWatcher) -> StatusOr<Self> {
        let config = SimpleConfigManager::from_config_resource(config_watcher, "text.conf")?;

        let (texture, mappings) = {
            let config = config.get();
            let fonts = file::util::resource_base().join("fonts");
            let packed = PackedGlyphSheet::new(config, &fonts)?;
            let texture = BitmapTexture::new(packed.image, config.texture_atlas_style, TextureUnit::Texture0);
            (texture, packed.mappings)
        };

        Ok(TextRenderer {
            config,
            texture,
            mappings,
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
                    self.mappings = packed.mappings;
                },
            }
        }
    }
}

