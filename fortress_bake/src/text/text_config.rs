use crate::{
    render::TextureStyle,
    text::{
        Locale,
        NamedText,
        RasterSize,
    },
};
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct TextConfig {
    pub current_locale: Locale,
    pub all_glyph_id_count_guess: usize,
    pub texture_atlas_size: (usize, usize),
    pub texture_atlas_style: TextureStyle,
    pub world_to_glyph_length_ratio: f32,
    pub raster_sizes: HashMap<RasterSize, f32>,
    pub text_sizes: HashMap<NamedText, Vec<RasterSize>>,
    pub localized_text: HashMap<Locale, HashMap<NamedText, String>>,
}
