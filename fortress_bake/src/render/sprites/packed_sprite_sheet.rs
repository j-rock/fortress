use crate::{
    app::StatusOr,
    render::{
        FramesInfo,
        NamedSpriteSheet,
        Png,
        SheetConfig,
        SpriteConfig,
        SpriteSheetFrameId,
    }
};
use rect_packer::DensePacker;
use std::path::PathBuf;

pub struct PackedSpriteSheet {
    pub image: Png,
    pub mappings: Vec<(SpriteSheetFrameId, FramesInfo)>
}

impl PackedSpriteSheet {
    pub fn new(config: &SheetConfig, images_dir: &PathBuf, sprite_sheet: NamedSpriteSheet) -> StatusOr<PackedSpriteSheet> {
        let sprite_sheet_dir = Self::base_directory(images_dir, sprite_sheet);
        let images = Self::read_images_with_identifiers(sprite_sheet_dir)?;

        let mut out_image = Png::empty(config.width, config.height);
        let mut mappings = Vec::with_capacity(images.len());

        let mut packer = DensePacker::new(config.width as i32, config.height as i32);
        for (name, image) in images.into_iter() {
            let (image_width, image_height) = image.size();
            let rect = packer.pack(image_width as i32, image_height as i32, false)
                .ok_or(format!("PackerSpriteSheetConfig too small for {:?}", sprite_sheet))?;

            out_image.overwrite(image, rect.x as usize, rect.y as usize)?;

            let frame_id = SpriteSheetFrameId {
                name,
                sprite_sheet,
            };
            let frame_info = match config.sprites.get(&frame_id.name) {
                None => {
                    let sprite = SpriteConfig {
                        frame_width: rect.width as usize,
                        frame_height: rect.height as usize,
                    };
                    FramesInfo::from_rect_pack(config, &sprite, rect)?
                },
                Some(sprite) => FramesInfo::from_rect_pack(config, sprite, rect)?,
            };
            mappings.push((frame_id, frame_info));
        }

        Ok(PackedSpriteSheet {
            image: out_image,
            mappings,
        })
    }

    fn base_directory(images_dir: &PathBuf, sprite_sheet: NamedSpriteSheet) -> PathBuf {
        images_dir.join(sprite_sheet.to_directory_basename())
    }

    fn read_images_with_identifiers(sprite_sheet_dir: PathBuf) -> StatusOr<Vec<(String, Png)>> {
        let mut images = vec!();
        for entry in sprite_sheet_dir.read_dir().map_err(|err| format!("Err reading dir {:?}: {}", sprite_sheet_dir, err))? {
            let entry = entry.map_err(|err| format!("Err reading entry: {}", err))?;
            let file_name = entry.file_name().into_string().map_err(|err| format!("Err retrieving filename: {:?}", err))?;
            images.push((file_name, Png::from_file(&entry.path())?));
        }

        images.sort_by(|a, b| {
            let (a_width, a_height) = a.1.size();
            let (b_width, b_height) = b.1.size();

            let a_size = a_width * a_height;
            let b_size = b_width * b_height;

            // Sort descending.
            b_size.cmp(&a_size)
        });

        Ok(images)
    }
}
