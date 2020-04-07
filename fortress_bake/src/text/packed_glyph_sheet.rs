use crate::{
    app::StatusOr,
    file,
    render::JsonBitmap,
    text::{
        GlyphId,
        GlyphInfo,
        TextConfig,
        TextSize,
    }
};
use font_atlas::{
    self,
    rasterize::{
        CharInfo,
        Font,
    },
};
use rect_packer::DensePacker;
use std::{
    collections::{
        HashMap,
        HashSet,
    },
    path::PathBuf,
};

pub struct PackedGlyphSheet {
    image: JsonBitmap,
    mappings: HashMap<GlyphId, GlyphInfo>,
}

impl PackedGlyphSheet {
    pub fn new(config: &TextConfig, fonts_dir: &PathBuf) -> StatusOr<Self> {
        let fonts = Self::load_all_fonts(fonts_dir)?;

        let mut atlas = JsonBitmap::empty(config.texture_atlas_size.0, config.texture_atlas_size.1);
        let mut packer = DensePacker::new(config.texture_atlas_size.0 as i32, config.texture_atlas_size.1 as i32);
        let mut mappings = HashMap::with_capacity(config.all_glyph_id_count_guess);

        for glyph in Self::all_glyphs(config) {
            let scale = config.raster_sizes.get(&glyph.size)
                .ok_or(format!("No raster size specified for {:?}", glyph.size))?;

            let (char_info, image) = Self::first_successful_raster(fonts.as_slice(), glyph.character, *scale)?;
            let (image_width, image_height) = image.size();
            let rect = packer.pack(image_width as i32, image_height as i32, false)
                .ok_or(String::from("Glyph texture atlas too small."))?;

            atlas.overwrite(image, rect.x as usize, rect.y as usize)?;
            mappings.insert(glyph, GlyphInfo::from(char_info, atlas.size(), rect));
        }

        Ok(PackedGlyphSheet {
            image: atlas,
            mappings,
        })
    }

    fn load_all_fonts(fonts_dir: &PathBuf) -> StatusOr<Vec<Font>> {
        let fonts =
            file::util::files_in_dir_ending_with(fonts_dir, ".ttf")?
            .iter()
            .filter_map(|base_name| {
                let path = fonts_dir.join(base_name);
                font_atlas::load_font(path).ok()
            })
            .collect();
        Ok(fonts)
    }

    fn all_glyphs(config: &TextConfig) -> impl Iterator<Item = GlyphId> {
        let mut all = HashSet::with_capacity(config.all_glyph_id_count_guess);

        for size in TextSize::all_sizes() {
            for character in "-0123456789".chars() {
                all.insert(GlyphId {
                    character,
                    size,
                });
            }
        }

        for text_mapping in config.localized_text.values() {
            for string in text_mapping.values() {
                for character in string.chars() {
                    all.insert(GlyphId {
                        character,
                        size: TextSize::Large,
                    });
                }
            }
        }

        all.into_iter()
    }

    fn first_successful_raster(fonts: &[Font], character: char, size: f32) -> StatusOr<(CharInfo, JsonBitmap)> {
        for font in fonts.iter() {
            if let Some((char_info, bitmap)) = font.render_char(character, size) {
                return Ok((char_info, JsonBitmap::from(bitmap)));
            }
        }
        Err(format!("No fonts worked for {}/{}", character, size))
    }
}
