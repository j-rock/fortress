use crate::{
    render::Texel,
    text::CharRasterInfo,
};
use glm;
use rect_packer::Rect;

pub struct GlyphInfo {
    texel: Texel,
}

impl GlyphInfo {
    pub fn from(_char_raster_info: CharRasterInfo, atlas_size: (usize, usize), rect: Rect) -> Self {
        let left_center = rect.x as f32 + 0.5;
        let top_center = (atlas_size.1 as i32 - rect.y) as f32 - 0.5;
        let texel_top_left = (left_center / atlas_size.0 as f32, top_center / atlas_size.1 as f32);
        let inner_width = (rect.width - 1) as f32 / atlas_size.0 as f32;
        let inner_height = (rect.height - 1) as f32 / atlas_size.1 as f32;
        let (left, top) = texel_top_left;
        let right = left + inner_width;
        let bottom = top - inner_height;
        let texel = Texel {
            bottom_left: glm::vec2(left, bottom),
            top_right: glm::vec2(right, top),
        };

        GlyphInfo {
            texel,
        }
    }

    pub fn texel(&self) -> Texel {
        self.texel
    }
}