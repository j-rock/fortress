use crate::render::Texel;
use font_atlas::rasterize::CharInfo;
use glm;
use rect_packer::Rect;

pub struct GlyphInfo {
    texel: Texel,
    draw_height_offset: f32,
    pre_draw_advance: glm::Vec2,
    post_draw_advance: glm::Vec2,
}

impl GlyphInfo {
    pub fn from(char_info: CharInfo, image_size: (usize, usize), rect: Rect) -> Self {
        let left_center = rect.x as f32 + 0.5;
        let top_center = (image_size.1 as i32 - rect.y) as f32 - 0.5;
        let texel_top_left = (left_center / image_size.0 as f32, top_center / image_size.1 as f32);
        let inner_width = (rect.width - 1) as f32 / image_size.0 as f32;
        let inner_height = (rect.height - 1) as f32 / image_size.1 as f32;
        let (left, top) = texel_top_left;
        let right = left + inner_width;
        let bottom = top - inner_height;
        let texel = Texel {
            bottom_left: glm::vec2(left, bottom),
            top_right: glm::vec2(right, top),
        };

        GlyphInfo {
            texel,
            draw_height_offset: char_info.height_offset,
            pre_draw_advance: glm::vec2(char_info.pre_draw_advance.0, char_info.pre_draw_advance.1),
            post_draw_advance: glm::vec2(char_info.post_draw_advance.0, char_info.post_draw_advance.1),
        }
    }

    pub fn texel(&self) -> Texel {
        self.texel
    }
}