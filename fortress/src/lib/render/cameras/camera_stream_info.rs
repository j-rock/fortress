use crate::{
    dimensions::{
        BoundingSquircle,
        GridIndex,
    },
    math::RandGen,
};
use nalgebra::{
    Matrix2,
    Point2,
    Vector2,
};

pub enum CameraStreamBounds {
    Outside,
    Margin(f32), // [0.0, 1.0] - 1.0 meaning almost inside.
    Inside
}

#[derive(Clone)]
pub struct CameraStreamInfo {
    inside_bounds: BoundingSquircle,
    margin_bounds: BoundingSquircle,
    light_bounds: BoundingSquircle,
    axial_to_cartesian: Matrix2<f64>,
    margin_length: f64,
}

impl CameraStreamInfo {
    pub fn new(center: Point2<f64>,
               inside_half_extents: Vector2<f64>,
               margin_length: f64,
               light_margin_length: f64,
               hex_cell_length: f64) -> Self {
        let axial_to_cartesian = GridIndex::axial_to_cartesian(hex_cell_length);

        let inside_bounds = BoundingSquircle::new(center.clone(), inside_half_extents.clone());
        let margin_half_extents = Vector2::new(margin_length, margin_length) + inside_half_extents;
        let margin_bounds = BoundingSquircle::new(center.clone(), margin_half_extents);
        let light_margin_half_extents = Vector2::new(light_margin_length, light_margin_length) + margin_half_extents;
        let light_bounds = BoundingSquircle::new(center, light_margin_half_extents);

        CameraStreamInfo {
            inside_bounds,
            margin_bounds,
            light_bounds,
            axial_to_cartesian,
            margin_length,
        }
    }

    pub fn is_point_inside(&self, point: Point2<f64>) -> bool {
        self.inside_bounds.contains(point)
    }

    pub fn is_point_outside_margin(&self, point: Point2<f64>) -> bool {
        !self.margin_bounds.contains(point)
    }

    pub fn is_point_within_light_margin(&self, point: Point2<f64>) -> bool {
        self.light_bounds.contains(point)
    }

    pub fn compute_bounds(&self, point: Point2<f64>) -> CameraStreamBounds {
        if self.is_point_outside_margin(point.clone()) {
            return CameraStreamBounds::Outside;
        }

        if self.is_point_inside(point.clone()) {
            return CameraStreamBounds::Inside;
        }

        let distance_from_inside = self.inside_bounds.distance_to(point);
        let analytical_distance = 1.0 - distance_from_inside / self.margin_length;
        let mut clamped = analytical_distance;
        if clamped < 0.0 {
            clamped = 0.0;
        } else if clamped > 1.0 {
            clamped = 1.0;
        }
        CameraStreamBounds::Margin(clamped as f32)
    }

    pub fn compute_grid_bounds(&self, grid_index: GridIndex) -> CameraStreamBounds {
        let cell_center = grid_index.index_center(&self.axial_to_cartesian);
        self.compute_bounds(cell_center)
    }

    pub fn random_point_inside_bounds(&self, rng: &mut RandGen) -> Point2<f64> {
        self.inside_bounds.random_point_inside(rng)
    }
}