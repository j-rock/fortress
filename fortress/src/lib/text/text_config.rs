use crate::text::{
    Locale,
    NamedText,
    TextSize,
};
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct TextConfig {
    pub texture_atlas_size: (u32, u32),
    pub warehouse_string_allocator_capacity: usize,
    pub current_locale: Locale,
    pub available_numeric_raster_sizes: Vec<TextSize>,
    pub available_text_sizes: HashMap<NamedText, Vec<TextSize>>,
    pub localized_text: HashMap<Locale, HashMap<NamedText, String>>,
}
