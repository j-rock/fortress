use crate::{
    app::StatusOr,
    file::{
        ConfigWatcher,
        SimpleConfigManager
    },
    render::{
        NamedSpriteSheet,
        PackedSpriteSheet,
        SpriteSheetConfig,
        SpriteSheetTexelId,
        Texel,
        Texture,
    },
};
use hashbrown::HashMap;

pub struct SpriteSheetTextureManager {
    config: SimpleConfigManager<SpriteSheetConfig>,

    textures: HashMap<NamedSpriteSheet, Texture>,
    texels: HashMap<SpriteSheetTexelId, Texel>,
}

impl SpriteSheetTextureManager {
    pub fn new(config_watcher: &mut ConfigWatcher) -> StatusOr<SpriteSheetTextureManager> {
        let config = SimpleConfigManager::<SpriteSheetConfig>::from_config_resource(config_watcher, "sprite_sheet.conf")?;

        let mut manager = SpriteSheetTextureManager {
            config,
            textures: HashMap::new(),
            texels: HashMap::new(),
        };
        manager.recompute_data()?;
        Ok(manager)
    }

    pub fn update(&mut self) {
        if self.config.update() {
            self.recompute_data().expect("Failed to update data!");
        }
    }

    pub fn recompute_data(&mut self) -> StatusOr<()> {
        self.textures.clear();
        self.texels.clear();

        let config = self.config.get();
        for sprite_sheet in NamedSpriteSheet::all_values() {
            let sheet_data = config.sheets.get(&sprite_sheet).ok_or(format!("No sheet data for {:?}", sprite_sheet))?;
            let packed = PackedSpriteSheet::new(sprite_sheet, sheet_data.width, sheet_data.height)?;
            self.textures.insert(sprite_sheet, Texture::new(packed.image, 0)?);

            for (sprite_sheet_texel_id, texel) in packed.mappings.into_iter() {
                self.texels.insert(sprite_sheet_texel_id, texel);
            }
        }

        Ok(())
    }

    pub fn texture(&self, sprite_sheet: NamedSpriteSheet) -> &Texture {
        self.textures.get(&sprite_sheet).expect("Missing texture!")
    }

    pub fn texel(&self, texel_id: &SpriteSheetTexelId) -> &Texel {
        self.texels.get(texel_id).expect("Missing texel id!")
    }
}
