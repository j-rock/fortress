use crate::{
    app::StatusOr,
    render::JsonBitmap,
    text::CharRasterInfo,
};
use rusttype;

pub struct Font<'a> {
    font: rusttype::Font<'a>,
}

impl<'a> Font<'a> {
    pub fn from_bytes(bytes: &[u8]) -> StatusOr<Font> {
        let font = rusttype::Font::from_bytes(bytes)
            .map_err(|e| format!("Font: {:?}", e))?;

        Ok(Font {
            font,
        })
    }

    pub fn from_vector(bytes: Vec<u8>) -> StatusOr<Font<'static>> {
        let font = rusttype::Font::from_bytes(bytes)
            .map_err(|e| format!("Font: {:?}", e))?;

        Ok(Font {
            font,
        })
    }

    pub fn render_char(&self, character: char, scale: f32) -> Option<(CharRasterInfo, JsonBitmap)> {
        let glyph = self.font.glyph(character)
            .scaled(rusttype::Scale::uniform(scale));
        let h_metrics = glyph.h_metrics();
        let glyph = glyph.positioned(rusttype::Point { x: 0.0, y:0.0 });
        let bb = glyph.pixel_bounding_box()?;
        let mut bitmap = JsonBitmap::empty(bb.width() as usize, bb.height() as usize);
        glyph.draw(|x, y, v| {
            bitmap.try_set_byte(x as usize, y as usize, (v * 255.0) as u8);
        });
        let info = CharRasterInfo {
            advance_width: h_metrics.advance_width,
            left_side_bearing: h_metrics.left_side_bearing,
            height_offset: glyph.position().y,
        };

        Some((info, bitmap))
    }
}