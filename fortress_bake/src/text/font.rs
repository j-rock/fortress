use crate::{
    app::StatusOr,
    render::SerializableBitmap,
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

    pub fn render_char(&self, character: char, scale: f32) -> Option<(CharRasterInfo, SerializableBitmap)> {
        let scale= rusttype::Scale::uniform(scale);
        let glyph = self.font.glyph(character).scaled(scale);
        let h_metrics = glyph.h_metrics();
        let glyph = glyph.positioned(rusttype::Point {
            x: 0.0,
            y: 0.0,
        });
        let bb = glyph.pixel_bounding_box()?;
        let mut bitmap = SerializableBitmap::empty(bb.width() as usize, bb.height() as usize);
        glyph.draw(|x, y, v| {
            bitmap.try_set_byte(x as usize, y as usize, (v * 255.0) as u8);
        });
        let info = CharRasterInfo {
            raster_dimensions: glm::vec2(bb.width() as f32, bb.height() as f32),
            advance_width: h_metrics.advance_width,
            left_side_bearing: h_metrics.left_side_bearing,
            height_offset: -bb.max.y as f32,
        };

        Some((info, bitmap))
    }
}