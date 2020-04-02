use crate::locales::{
   LocalizedTextMapping,
   NamedLocale,
};
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct TextConfig {
    pub initial_glyph_cache_size: (u32, u32),
    pub warehouse_string_allocator_capacity: usize,
    pub current_locale: NamedLocale,
    pub locale_text: HashMap<NamedLocale, LocalizedTextMapping>,
}
