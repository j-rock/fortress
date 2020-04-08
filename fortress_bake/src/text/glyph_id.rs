use crate::text::RasterSize;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Hash)]
pub struct GlyphId {
    pub character: char,
    pub size: RasterSize,
}

impl GlyphId {
    pub fn new(character: char, size: RasterSize) -> Self {
        GlyphId {
            character,
            size,
        }
    }
}
