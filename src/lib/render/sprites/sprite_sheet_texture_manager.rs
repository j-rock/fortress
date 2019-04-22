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
        SpriteSheetFrameId,
        FrameInfo,
        Texel,
        Texture,
    },
};
use hashbrown::HashMap;

pub struct SpriteSheetTextureManager {
    config: SimpleConfigManager<SpriteSheetConfig>,

    textures: HashMap<NamedSpriteSheet, Texture>,
    frames: HashMap<SpriteSheetFrameId, FrameInfo>,
}

impl SpriteSheetTextureManager {
    pub fn new(config_watcher: &mut ConfigWatcher) -> StatusOr<SpriteSheetTextureManager> {
        let config = SimpleConfigManager::<SpriteSheetConfig>::from_config_resource(config_watcher, "sprite_sheet.conf")?;

        let mut manager = SpriteSheetTextureManager {
            config,
            textures: HashMap::new(),
            frames: HashMap::new(),
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
        self.frames.clear();

        let config = self.config.get();
        for sprite_sheet in NamedSpriteSheet::all_values() {
            let sheet_config = config.sheets.get(&sprite_sheet).ok_or(format!("No sheet data for {:?}", sprite_sheet))?;
            let packed = PackedSpriteSheet::new(sheet_config, sprite_sheet)?;
            self.textures.insert(sprite_sheet, Texture::new(packed.image, 0));

            for (sprite_sheet_frame_id, frame_info) in packed.mappings.into_iter() {
                self.frames.insert(sprite_sheet_frame_id, frame_info);
            }
        }

        Ok(())
    }

    pub fn texture(&self, sprite_sheet: NamedSpriteSheet) -> &Texture {
        self.textures.get(&sprite_sheet).expect("Missing texture!")
    }

    pub fn frame(&self, frame_id: &SpriteSheetFrameId, frame: usize) -> Texel {
        let frame_info = self.frames.get(frame_id).expect("Missing frame id!");
        frame_info.texel(frame)
    }
}
