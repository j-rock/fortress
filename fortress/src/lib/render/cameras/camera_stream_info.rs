use crate::dimensions::{
    BoundingBox2,
    BoundingBoxOverlap,
    GridIndex,
};
use nalgebra::{
    Matrix2,
    Point2,
    Vector2,
};

pub enum CameraStreamBounds {
    Outside,
    Margin(f64), // [0.0, 1.0)
    Inside
}

pub struct CameraStreamInfo {
    inside_bounds: BoundingBox2,
    margin_bounds: BoundingBox2,
    axial_to_cartesian: Matrix2<f64>,
    margin_length: f64,
    hex_cell_length: f64,
}

impl CameraStreamInfo {
    pub fn new(center: Point2<f64>,
               inside_half_extents: Vector2<f64>,
               margin_length: f64,
               hex_cell_length: f64) -> Self {
        let axial_to_cartesian = GridIndex::axial_to_cartesian(hex_cell_length);

        let inside_bounds = BoundingBox2::new(center.clone() - inside_half_extents.clone(), center.clone() + inside_half_extents.clone());
        let margin_half_extents = Vector2::new(margin_length, margin_length) + inside_half_extents;
        let margin_bounds = BoundingBox2::new(center.clone() - margin_half_extents.clone(), center + margin_half_extents);

        CameraStreamInfo {
            inside_bounds,
            margin_bounds,
            axial_to_cartesian,
            margin_length,
            hex_cell_length,
        }
    }

    pub fn compute_bounds(&self, grid_index: GridIndex) -> CameraStreamBounds {
        let cell_bounds = grid_index.bounding_box(self.hex_cell_length, &self.axial_to_cartesian);

        if let BoundingBoxOverlap::Touching = self.inside_bounds.overlap_with(cell_bounds) {
            return CameraStreamBounds::Inside;
        }

        if let BoundingBoxOverlap::Disjoint = self.margin_bounds.overlap_with(cell_bounds) {
            return CameraStreamBounds::Outside;
        }

        let distance_from_inside = cell_bounds.min_distance_to(self.inside_bounds);
        CameraStreamBounds::Margin(distance_from_inside / self.margin_length)
    }
}