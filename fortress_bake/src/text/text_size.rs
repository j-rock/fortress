use enum_iterator::IntoEnumIterator;

#[derive(Copy, Clone, Deserialize, Debug, PartialEq, Eq, PartialOrd, Hash, IntoEnumIterator)]
pub enum TextSize {
    Small, Medium, Large
}

impl TextSize {
    pub fn all_sizes() -> <Self as IntoEnumIterator>::Iterator {
        Self::into_enum_iter()
    }
}
