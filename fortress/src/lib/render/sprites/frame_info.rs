use crate::{
    dimensions::Reverse,
    render::Texel
};
use glm;

#[derive(Copy, Clone, Debug)]
pub struct FrameInfo {
    pub bottom_left: glm::Vec2,
    pub top_right: glm::Vec2,
    pub num_sub_frames_horizontal: usize,
    pub num_sub_frames_vertical: usize,
    pub sub_frame_width: f32,
    pub sub_frame_height: f32,
}

impl FrameInfo {
    pub fn texel(&self, frame: usize, reverse: Reverse) -> Texel {
        let frame = frame % (self.num_sub_frames_horizontal * self.num_sub_frames_vertical);
        let frame_x = (frame % self.num_sub_frames_horizontal) as f32;
        let frame_y = (frame / self.num_sub_frames_horizontal) as f32;

        let mut sub_frame_left = self.bottom_left.x + frame_x * self.sub_frame_width;
        let mut sub_frame_right = self.bottom_left.x + (frame_x + 1.0) * self.sub_frame_width;
        let mut sub_frame_top = self.top_right.y - frame_y * self.sub_frame_height;
        let mut sub_frame_bottom = self.top_right.y - (frame_y + 1.0) * self.sub_frame_height;

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

