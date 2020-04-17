pub struct EasingFn;

impl EasingFn {
    pub fn ease_out_quad(t: f32) -> f32 {
        t * (2.0 - t)
    }

    pub fn ease_out_quintic(t: f32) -> f32 {
        1.0 + Self::ease_in_quintic(t - 1.0)
    }

    pub fn ease_in_quad(t: f32) -> f32 {
        t * t
    }

    pub fn ease_in_cubic(t: f32) -> f32 {
        t * t * t
    }

    pub fn ease_in_cuartic(t: f32) -> f32 {
        let tt = t * t;
        tt * tt
    }

    pub fn ease_in_quintic(t: f32) -> f32 {
        t * Self::ease_in_cuartic(t)
    }
}
