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
    pub raster_sizes: HashMap<RasterSize, f32>,
    pub localized_text: HashMap<Locale, HashMap<NamedText, String>>,
}
