use enum_iterator::Sequence;

#[derive(Copy, Clone, Deserialize, Debug, PartialEq, Eq, PartialOrd, Hash, Sequence)]
pub enum RasterSize {
    Small,
    Medium,
    Large,
}

impl RasterSize {
    pub fn all_values() -> impl Iterator<Item = Self> {
        enum_iterator::all::<Self>()
    }
}
