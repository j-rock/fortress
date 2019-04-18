use crate::{
    app::StatusOr,
    file,
    image::Png,
    render::{
        NamedSpriteSheet,
        SpriteSheetTexelId,
        Texel,
    }
};
use glm;
use rect_packer::{
    DensePacker,
    Rect,
};
use std::path::PathBuf;

pub struct PackedSpriteSheet {
    pub image: Png,
    pub mappings: Vec<(SpriteSheetTexelId, Texel)>
}

impl PackedSpriteSheet {
    pub fn new(sprite_sheet: NamedSpriteSheet, width: usize, height: usize) -> StatusOr<PackedSpriteSheet> {
        let images_dir = Self::base_directory(sprite_sheet);
        let images = Self::read_images_with_identifiers(images_dir)?;

        let mut out_image = Png::empty(width, height);
        let mut mappings = Vec::with_capacity(images.len());

        let mut packer = DensePacker::new(width as i32, height as i32);
        for (name, image) in images.into_iter() {
            let (image_width, image_height) = image.size();
            let rect = packer.pack(image_width as i32, image_height as i32, false)
                .ok_or(format!("PackerSpriteSheetConfig too small for {:?}", sprite_sheet))?;

            out_image.overwrite(image, rect.x as usize, rect.y as usize)?;

            let texel_id = SpriteSheetTexelId {
                name,
                sprite_sheet,
            };
            mappings.push((texel_id, Self::compute_texel(width as i32, height as i32, rect)));
        }

        Ok(PackedSpriteSheet {
            image: out_image,
            mappings,
        })
    }

    fn base_directory(sprite_sheet: NamedSpriteSheet) -> PathBuf {
        let mut images_dir = file::util::resource_base();
        images_dir.push("images");
        images_dir.push(sprite_sheet.to_lowercase_string());
        images_dir
    }

    fn read_images_with_identifiers(images_dir: PathBuf) -> StatusOr<Vec<(String, Png)>> {
        let mut images = vec!();
        for entry in images_dir.read_dir().map_err(|err| format!("Err reading dir {:?}: {}", images_dir, err))? {
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

    fn compute_texel(image_width: i32, image_height: i32, rect: Rect) -> Texel {
        let bottom_left_pixel = (rect.x, image_height - (rect.y + rect.height - 1));
        let top_right_pixel = (rect.x + rect.width - 1, image_height - rect.y);

        let bottom_left = glm::vec2(
            bottom_left_pixel.0 as f32 / image_width as f32,
            bottom_left_pixel.1 as f32 / image_height as f32);

        let top_right = glm::vec2(
            top_right_pixel.0 as f32 / image_width as f32,
            top_right_pixel.1 as f32 / image_height as f32);

        Texel {
            bottom_left,
            top_right,
        }
    }
}

