use crate::{
    app::StatusOr,
    dimensions::Reverse,
    file::{
        ConfigWatcher,
        SimpleConfigManager,
        self,
    },
    render::{
        AllPackedSpriteSheets,
        FramesInfo,
        NamedSpriteSheet,
        SpriteSheetConfig,
        SpriteSheetFrameId,
        Texel,
        Texture,
    },
};
use std::collections::HashMap;

pub struct SpriteSheetTextureManager {
    config: SimpleConfigManager<SpriteSheetConfig>,

    textures: HashMap<NamedSpriteSheet, Texture>,
    frames: HashMap<SpriteSheetFrameId, FramesInfo>,
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
        let images_dir = file::util::resource_base().join("images");
        let all_sheets = AllPackedSpriteSheets::read_from_files(config, &images_dir)?;

        for (sprite_sheet, (image, texture_style)) in all_sheets.images.into_iter() {
            self.textures.insert(sprite_sheet, Texture::new(image, texture_style, 0));
        }
        for (frame_id, frames_info) in all_sheets.frames.into_iter() {
           self.frames.insert(frame_id, frames_info);
        }

        Ok(())
    }

    pub fn texture(&self, sprite_sheet: NamedSpriteSheet) -> &Texture {
        self.textures.get(&sprite_sheet).expect("Missing texture!")
    }

    pub fn frame(&self, frame_id: &SpriteSheetFrameId, frame: usize, reverse: Reverse) -> Texel {
        let frame_info = self.frames.get(frame_id).expect("Missing frame id!");
        frame_info.texel(frame, reverse)
    }
}
