use enum_iterator::IntoEnumIterator;

#[derive(Copy, Clone, Deserialize, Debug, PartialEq, Eq, PartialOrd, Hash, IntoEnumIterator)]
pub enum RasterSize {
    Small,
    Medium,
    Large,
}

impl RasterSize {
    pub fn all_values() -> impl Iterator<Item = Self> {
        Self::into_enum_iter()
    }
}
