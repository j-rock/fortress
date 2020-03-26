use crate::{
    app::StatusOr,
    render::{
        FramesInfo,
        NamedSpriteSheet,
        PackedSpriteSheet,
        Png,
        SpriteSheetConfig,
        SpriteSheetFrameId,
        TextureStyle,
    }
};
use std::{
    collections::HashMap,
    path::PathBuf,
};

pub struct AllPackedSpriteSheets {
    pub images: HashMap<NamedSpriteSheet, (Png, TextureStyle)>,
    pub frames: HashMap<SpriteSheetFrameId, FramesInfo>,
}

impl AllPackedSpriteSheets {
    pub fn new(images: HashMap<NamedSpriteSheet, (Png, TextureStyle)>, frames: HashMap<SpriteSheetFrameId, FramesInfo>) -> Self {
        AllPackedSpriteSheets {
            images,
            frames,
        }
    }

    pub fn read_from_files(config: &SpriteSheetConfig, images_dir: &PathBuf) -> StatusOr<Self> {
        let mut images = HashMap::new();
        let mut frames =  HashMap::new();

        for sprite_sheet in NamedSpriteSheet::all_values().into_iter() {
            let sheet_config = config.sheets.get(&sprite_sheet).ok_or(format!("No sheet data for {:?}", sprite_sheet))?;
            let packed = PackedSpriteSheet::new(sheet_config, &images_dir, sprite_sheet)?;
            images.insert(sprite_sheet, (packed.image, sheet_config.style));
            for (sprite_sheet_frame_id, frames_info) in packed.mappings.into_iter() {
                frames.insert(sprite_sheet_frame_id, frames_info);
            }
        }

        Ok(AllPackedSpriteSheets {
            images,
            frames,
        })
    }
}

