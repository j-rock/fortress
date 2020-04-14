use crate::text::{
    GlyphId,
    GlyphInfo,
    Locale,
    NamedText,
    TextConfig,
};
use std::collections::HashMap;

pub struct TextResolver {
    localized_text: HashMap<(Locale, NamedText), String>,
    mappings: HashMap<GlyphId, GlyphInfo>,
}

impl TextResolver {
    pub fn new(config: &TextConfig, mappings: HashMap<GlyphId, GlyphInfo>) -> Self {
        let localized_text =
            config.localized_text
                .iter()
                .flat_map(|(locale, named_text_map)| {
                    let locale = *locale;
                    named_text_map
                        .iter()
                        .map(move |(named_text, text)| {
                            ((locale, *named_text), text.clone())
                        })
                })
                .collect();

        TextResolver {
            localized_text,
            mappings,
        }
    }

    pub fn get_text(&self, current_locale: Locale, text: NamedText) -> Option<&String> {
        self.localized_text.get(&(current_locale, text))
    }

    pub fn get_glyph_info(&self, glyph_id: GlyphId) -> Option<&GlyphInfo> {
        self.mappings.get(&glyph_id)
    }
}