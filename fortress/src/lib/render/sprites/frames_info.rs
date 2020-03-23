use crate::{
    dimensions::Reverse,
    render::Texel
};
use glm;

#[derive(Copy, Clone, Debug)]
pub struct FramesInfo {
    pub texel_top_left: glm::Vec2,

    pub num_frames_horizontal: usize,
    pub num_frames_vertical: usize,

    // Frame {width,height} correspond to distances between frames.
    pub frame_width: f32,
    pub frame_height: f32,

    // Frame sub-{width,height} correspond to distances within a single frame.
    pub sub_frame_width: f32,
    pub sub_frame_height: f32,
}

impl FramesInfo {
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

