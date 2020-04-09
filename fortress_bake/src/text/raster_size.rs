use enum_iterator::IntoEnumIterator;

#[derive(Copy, Clone, Deserialize, Debug, PartialEq, Eq, PartialOrd, Hash, IntoEnumIterator)]
pub enum RasterSize {
    // Keep in sorted order from largest to smallest.
    Large,
    Medium,
    Small,
}

impl RasterSize {
    pub fn largest_to_smallest() -> impl Iterator<Item = Self> {
        Self::into_enum_iter()
    }
}
