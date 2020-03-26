use crate::{
    app::StatusOr,
    dimensions::Reverse,
    file::ConfigWatcher,
    render::{
        AllPackedSpriteSheets,
        FramesInfo,
        NamedSpriteSheet,
        SpriteSheetFrameId,
        Texel,
        Texture,
    },
};
#[cfg(not(feature = "bake"))]
use crate::{
    file::{
        SimpleConfigManager,
        self,
    },
    render::SpriteSheetConfig,
};
#[cfg(feature = "bake")]
use crate::render::{
    BakedSpriteSheetConfig,
    Png,
};
use std::collections::HashMap;

pub struct SpriteSheetTextureManager {
    #[cfg(not(feature = "bake"))]
    config: SimpleConfigManager<SpriteSheetConfig>,
    textures: HashMap<NamedSpriteSheet, Texture>,
    frames: HashMap<SpriteSheetFrameId, FramesInfo>,
}

impl SpriteSheetTextureManager {
    #[cfg(not(feature = "bake"))]
    pub fn new(config_watcher: &mut ConfigWatcher) -> StatusOr<Self> {
        let config = SimpleConfigManager::<SpriteSheetConfig>::from_config_resource(config_watcher, "sprite_sheet.conf")?;
        let mut manager = SpriteSheetTextureManager {
            config,
            textures: HashMap::new(),
            frames: HashMap::new(),
        };
        manager.recompute_data()?;
        Ok(manager)
    }

    #[cfg(feature = "bake")]
    pub fn new(_config_watcher: &mut ConfigWatcher) -> StatusOr<Self> {
        let mut manager = SpriteSheetTextureManager {
            textures: HashMap::new(),
            frames: HashMap::new(),
        };
        manager.recompute_data()?;
        Ok(manager)
    }

    #[cfg(not(feature = "bake"))]
    pub fn update(&mut self) {
        if self.config.update() {
            self.recompute_data().expect("Failed to update data!");
        }
    }

    #[cfg(feature = "bake")]
    pub fn update(&mut self) {}

    pub fn texture(&self, sprite_sheet: NamedSpriteSheet) -> &Texture {
        self.textures.get(&sprite_sheet).expect("Missing texture!")
    }

    pub fn frame(&self, frame_id: &SpriteSheetFrameId, frame: usize, reverse: Reverse) -> Texel {
        let frame_info = self.frames.get(frame_id).expect("Missing frame id!");
        frame_info.texel(frame, reverse)
    }

    #[cfg(not(feature = "bake"))]
    fn recompute_data(&mut self) -> StatusOr<()> {
        let config = self.config.get();
        let images_dir = file::util::resource_base().join("images");
        let all_sheets = AllPackedSpriteSheets::read_from_files(config, &images_dir)?;
        self.reset_from(all_sheets);
        Ok(())
    }

    #[cfg(feature = "bake")]
    fn recompute_data(&mut self) -> StatusOr<()> {
        let baked_sprite_sheet_config = include_bytes!(concat!(env!("OUT_DIR"), "\\config\\sprite_sheet.conf"));
        let galaxy_ground = include_bytes!(concat!(env!("OUT_DIR"), "\\images\\GalaxyGround.png"));
        let heroes = include_bytes!(concat!(env!("OUT_DIR"), "\\images\\Heroes.png"));
        let sprite_sheet1 = include_bytes!(concat!(env!("OUT_DIR"), "\\images\\SpriteSheet1.png"));

        let baked_config: BakedSpriteSheetConfig =
            serde_json::from_slice(baked_sprite_sheet_config)
                .map_err(|e| format!("{:?}", e))?;

        let mut images = HashMap::new();
        for (named_sprite_sheet, texture_style) in baked_config.images.into_iter() {
            let image = Png::from_slice(match named_sprite_sheet {
                NamedSpriteSheet::SpriteSheet1 => sprite_sheet1,
                NamedSpriteSheet::GalaxyGround => galaxy_ground,
                NamedSpriteSheet::Heroes => heroes,
            })?;
            images.insert(named_sprite_sheet, (image, texture_style));
        }
        let frames = baked_config.frames.into_iter().collect();

        let all_sheets = AllPackedSpriteSheets::new(images, frames);
        self.reset_from(all_sheets);
        Ok(())
    }

    fn reset_from(&mut self, all_sheets: AllPackedSpriteSheets) {
        self.textures.clear();
        self.frames.clear();

        for (sprite_sheet, (image, texture_style)) in all_sheets.images.into_iter() {
            self.textures.insert(sprite_sheet, Texture::new(image, texture_style, 0));
        }

        for (frame_id, frames_info) in all_sheets.frames.into_iter() {
            self.frames.insert(frame_id, frames_info);
        }
    }
}
