pub struct EasingFn;

impl EasingFn {
    pub fn ease_out_quad(t: f32) -> f32 {
        t * (2.0 - t)
    }

    pub fn ease_in_cuartic(t: f32) -> f32 {
        let tt = t * t;
        tt * tt
    }
}
