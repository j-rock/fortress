pub struct EasingFn;

impl EasingFn {
    pub fn ease_out_quad(t: f32) -> f32 {
        t * (2.0 - t)
    }
}
