use crate::{
    app::StatusOr,
    dimensions::Reverse,
    render::{
        SheetConfig,
        SpriteConfig,
        Texel,
    },
};
use glm;
use rect_packer::Rect;

#[derive(Copy, Clone, Debug)]
pub struct FramesInfo {
    num_frames_horizontal: usize,
    num_frames_vertical: usize,
    texel_top_left: glm::Vec2,

    // Frame {width,height} correspond to distances between frames.
    frame_width: f32,
    frame_height: f32,

    // Frame sub-{width,height} correspond to distances within a single frame.
    sub_frame_width: f32,
    sub_frame_height: f32,
}

impl FramesInfo {
    pub fn from_rect_pack(config: &SheetConfig, sprite: &SpriteConfig, rect: Rect) -> StatusOr<Self> {
        if rect.width == 0 || rect.height == 0 {
            return Err(format!("Bad FramesInfo rect: {}, {}", rect.width as usize, sprite.frame_width));
        }

        let num_frames_horizontal = (rect.width as usize) / sprite.frame_width;
        let num_frames_vertical = (rect.height as usize) / sprite.frame_height;

        let left_center = rect.x as f32 + 0.5;
        let top_center = (config.height as i32 - rect.y) as f32 - 0.5;
        let texel_top_left = glm::vec2(left_center / config.width as f32, top_center / config.height as f32);

        let frame_width = sprite.frame_width as f32 / config.width as f32;
        let frame_height = sprite.frame_height as f32 / config.height as f32;
        let sub_frame_width = (sprite.frame_width - 1) as f32 / config.width as f32;
        let sub_frame_height = (sprite.frame_height - 1) as f32 / config.height as f32;

        Ok(FramesInfo {
            num_frames_horizontal,
            num_frames_vertical,
            texel_top_left,
            frame_width,
            frame_height,
            sub_frame_width,
            sub_frame_height,
        })
    }

    pub fn texel(&self, frame: usize, reverse: Reverse) -> Texel {
        let frame = frame % (self.num_frames_horizontal * self.num_frames_vertical);
        let frame_x = (frame % self.num_frames_horizontal) as f32;
        let frame_y = (frame / self.num_frames_horizontal) as f32;

        let mut sub_frame_left = self.texel_top_left.x + frame_x * self.frame_width;
        let mut sub_frame_right = sub_frame_left + self.sub_frame_width;
        let mut sub_frame_top = self.texel_top_left.y - frame_y * self.frame_height;
        let mut sub_frame_bottom = sub_frame_top - self.sub_frame_height;

        if reverse.horizontally {
           std::mem::swap(&mut sub_frame_left, &mut sub_frame_right);
        }
        if reverse.vertically {
            std::mem::swap(&mut sub_frame_bottom, &mut sub_frame_top);
        }

        Texel {
            bottom_left: glm::vec2(sub_frame_left, sub_frame_bottom),
            top_right: glm::vec2(sub_frame_right, sub_frame_top),
        }
    }
}

