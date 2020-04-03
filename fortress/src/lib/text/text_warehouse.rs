use crate::{
    data::{
        StringAllocator,
        UnsafeStr,
    },
    text::{
        Locale,
        NamedText,
        TextConfig,
    },
};
use std::collections::HashMap;

pub struct TextWarehouse {
    string_allocator: StringAllocator,
    localized_text: HashMap<(Locale, NamedText), String>,
}

impl TextWarehouse {
    pub fn new(config: &TextConfig) -> Self {
        let localized_text =
            config.localized_text
            .iter()
            .flat_map(|(locale, named_text_map)| {
                named_text_map
                    .iter()
                    .map(|(named_text, text)| {
                        ((locale, named_text), text.clone())
                    })
            })
            .collect();

        TextWarehouse {
            string_allocator: StringAllocator::with_capacity(config.warehouse_string_allocator_capacity),
            localized_text,
        }
    }

    pub fn clear_string_allocator(&mut self) {
        self.string_allocator.clear();
    }

    pub fn get_number(&mut self, val: i64) -> Option<UnsafeStr> {
        let number_string = format!("{}", val);
        let s = self.string_allocator.allocate(number_string)?;
        Some(UnsafeStr::from(s))
    }

    pub fn get_text(&self, locale: Locale, named_text: NamedText) -> Option<&str> {
        self.localized_text
            .get(&(locale, named_text))
            .map(|string| string.as_str())
    }
}