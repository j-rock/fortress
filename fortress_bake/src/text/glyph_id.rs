use crate::text::TextSize;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Hash)]
pub struct GlyphId {
    pub character: char,
    pub size: TextSize,
}
