use crate::render::{
    FramesInfo,
    NamedSpriteSheet,
    SpriteSheetConfig,
    SpriteSheetFrameId,
    TextureStyle,
};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct BakedSpriteSheetConfig {
    pub images: Vec<(NamedSpriteSheet, TextureStyle)>,
    pub frames: Vec<(SpriteSheetFrameId, FramesInfo)>,
}

impl BakedSpriteSheetConfig {
    pub fn new(config: SpriteSheetConfig, frames: HashMap<SpriteSheetFrameId, FramesInfo>) -> Self {
        let images = config.sheets
            .into_iter()
            .map(|(named_sprite_sheet, sheet_config)| {
                (named_sprite_sheet, sheet_config.style)
            })
            .collect();

        let frames = frames.into_iter().collect();

        BakedSpriteSheetConfig {
            images,
            frames,
        }
    }
}
