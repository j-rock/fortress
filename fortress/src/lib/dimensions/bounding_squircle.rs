use nalgebra::{
    Point2,
    Vector2,
};

// (x-h)^4   (y-k)^4
// ------- + ------- = 1
//   a^4       b^4
#[derive(Clone)]
pub struct BoundingSquircle {
    center: Point2<f64>,
    half_extents: Vector2<f64>,
}

impl BoundingSquircle {
    pub fn new(center: Point2<f64>, half_extents: Vector2<f64>) -> Self {
        BoundingSquircle {
            center,
            half_extents,
        }
    }

    fn fourth_power(t: f64) -> f64 {
        let t = t * t;
        t * t
    }

    pub fn contains(&self, point: Point2<f64>) -> bool {
        let xcomp = (point.x - self.center.x) / self.half_extents.x;
        let ycomp = (point.y - self.center.y) / self.half_extents.y;

        Self::fourth_power(xcomp) + Self::fourth_power(ycomp) <= 1.0
    }

    pub fn distance_to(&self, point: Point2<f64>) -> f64 {
        if self.contains(point.clone()) {
            return 0.0;
        }

        let d = point - self.center;
        let numerator = Self::fourth_power(self.half_extents.x * self.half_extents.y);
        let denominator = Self::fourth_power(self.half_extents.y * d.x) + Self::fourth_power(self.half_extents.x * d.y);

        let t = (numerator / denominator).sqrt().sqrt();
        (1.0 - t) * d.norm()
    }
}
