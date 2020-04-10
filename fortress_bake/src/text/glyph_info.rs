use crate::{
    render::Texel,
    text::CharRasterInfo,
};
use glm;
use rect_packer::Rect;

pub struct GlyphInfo {
    texel: Texel,
    char_raster_info: CharRasterInfo,
}

impl GlyphInfo {
    pub fn from(char_raster_info: CharRasterInfo, atlas_size: (usize, usize), rect: Rect) -> Self {
        let left_center = rect.x as f32 + 0.5;
        let bottom_center = (atlas_size.1 as i32 - rect.y) as f32 - 0.5;
        let texel_bottom_left = (left_center / atlas_size.0 as f32, bottom_center / atlas_size.1 as f32);
        let inner_width = (rect.width - 1) as f32 / atlas_size.0 as f32;
        let inner_height = (rect.height - 1) as f32 / atlas_size.1 as f32;
        let (left, bottom) = texel_bottom_left;
        let right = left + inner_width;
        let top = bottom - inner_height;
        let texel = Texel {
            // Invert y coords because OpenGl loads bitmap data weird.
            bottom_left: glm::vec2(left, 1.0 - bottom),
            top_right: glm::vec2(right, 1.0 - top),
        };

        GlyphInfo {
            texel,
            char_raster_info,
        }
    }

    pub fn texel(&self) -> Texel {
        self.texel
    }

    pub fn raster_info(&self) -> &CharRasterInfo {
        &self.char_raster_info
    }
}