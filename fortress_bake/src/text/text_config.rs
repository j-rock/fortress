use crate::{
    render::TextureStyle,
    text::{
        Locale,
        NamedText,
        TextSize,
    },
};
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct TextConfig {
    pub texture_atlas_size: (usize, usize),
    pub texture_atlas_style: TextureStyle,
    pub all_glyph_id_count_guess: usize,
    pub warehouse_string_allocator_capacity: usize,
    pub current_locale: Locale,
    pub raster_sizes: HashMap<TextSize, f32>,
    pub localized_text: HashMap<Locale, HashMap<NamedText, String>>,
}
