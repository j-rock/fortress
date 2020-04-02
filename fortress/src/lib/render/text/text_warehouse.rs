use crate::{
    data::{
        StringAllocator,
        UnsafeStr,
    },
    locales::{
        NamedLocale,
        LocalizedTextKey,
        LocalizedTextMapping,
    },
    render::TextConfig,
};
use std::collections::HashMap;

pub struct TextWarehouse {
    string_allocator: StringAllocator,
    localized_text: HashMap<NamedLocale, LocalizedTextMapping>,
}

impl TextWarehouse {
    pub fn new(config: &TextConfig) -> Self {
        TextWarehouse {
            string_allocator: StringAllocator::with_capacity(config.warehouse_string_allocator_capacity),
            localized_text: config.locale_text.clone(),
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

    pub fn get_text(&self, locale: NamedLocale, key: LocalizedTextKey) -> Option<&str> {
        self.localized_text.get(&locale)?.get(&key)
    }
}