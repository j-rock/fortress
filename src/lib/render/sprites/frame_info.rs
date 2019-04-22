use crate::render::Texel;
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
    pub fn texel(&self, frame: usize) -> Texel {
        let frame_x = (frame % self.num_sub_frames_horizontal) as f32;
        let frame_y = (frame / self.num_sub_frames_horizontal) as f32;

        let sub_frame_left = self.bottom_left.x + frame_x * self.sub_frame_width;
        let sub_frame_right = self.bottom_left.x + (frame_x + 1.0) * self.sub_frame_width - 0.00001;
        let sub_frame_top = self.top_right.y - frame_y * self.sub_frame_height;
        let sub_frame_bottom = self.top_right.y - (frame_y + 1.0) * self.sub_frame_height + 0.00001;

        Texel {
            bottom_left: glm::vec2(sub_frame_left, sub_frame_bottom),
            top_right: glm::vec2(sub_frame_right, sub_frame_top),
        }
    }
}

