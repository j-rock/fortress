use nalgebra::Point2;

pub enum BoundingBoxOverlap {
    Disjoint,
    Touching,
}

#[derive(Copy, Clone)]
pub struct BoundingBox2 {
    min: Point2<f64>,
    max: Point2<f64>,
}

impl BoundingBox2 {
    pub fn new(min: Point2<f64>, max: Point2<f64>) -> Self {
        BoundingBox2 {
            min,
            max,
        }
    }

    pub fn min(&self) -> Point2<f64> {
        *&self.min
    }

    pub fn max(&self) -> Point2<f64> {
        *&self.max
    }

    pub fn overlap_with(&self, other: BoundingBox2) -> BoundingBoxOverlap {
        let x_overlap =
            (self.min.x <= other.max.x && self.max.x >= other.min.x) ||
            (other.min.x <= self.max.x && other.max.x >= self.min.x);
        if !x_overlap {
            return BoundingBoxOverlap::Disjoint;
        }

        let y_overlap =
            (self.min.y <= other.max.y && self.max.y >= other.min.y) ||
                (other.min.y <= self.max.y && other.max.y >= self.min.y);
        if !y_overlap {
            return BoundingBoxOverlap::Disjoint;
        }

        BoundingBoxOverlap::Touching
    }

    pub fn min_distance_to(&self, other: BoundingBox2) -> f64 {
        let candidates: [f64; 4] = [
            self.min.y - other.max.y,
            other.min.y - self.max.y,
            self.min.x - other.max.x,
            other.min.x - self.max.x,
        ];
        let mut minimum = None;
        for candidate in candidates.iter().cloned() {
            if candidate < 0.0 {
                continue;
            }
            match minimum {
                None => {
                    minimum = Some(candidate);
                },
                Some(champion) if candidate < champion => {
                    minimum = Some(candidate);
                },
                _ => {},
            }
        }
        minimum.unwrap_or(0.0)
    }
}