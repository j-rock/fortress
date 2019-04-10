use nalgebra::{
    Matrix2,
    RowVector2,
    Vector2,
};
use ncollide2d::{
    math::Point,
    shape::Segment
};

static DIRECTIONS: &[GridDirection] = &[
    GridDirection { q: -1, r: 0 },
    GridDirection { q: -1, r: 1 },
    GridDirection { q:  0, r: 1 },
    GridDirection { q:  1, r: 0 },
    GridDirection { q: 1, r: -1 },
    GridDirection { q: 0, r: -1 },
];

static HALF_ROOT_3: f64 = 0.8660254037844386;

#[derive(Debug)]
pub struct GridDirection {
    q: i64,
    r: i64,
}

impl GridDirection {
    pub fn all() -> &'static [GridDirection] {
        DIRECTIONS
    }

    // Returns direction vectors from a hex cell center to the vertices that define the hexagon edge
    // in the direction specified by this GridDirection.
    pub fn cartesian_offsets(&self, hex_side_length: f64) -> (Vector2<f64>, Vector2<f64>) {
        match self {
            GridDirection { q: -1, r: 0 } => {
                (Vector2::new(-hex_side_length, 0.0),
                 Vector2::new(-hex_side_length / 2.0, HALF_ROOT_3 * hex_side_length))
            },
            GridDirection { q: -1, r: 1 } => {
                (Vector2::new(-hex_side_length / 2.0, -HALF_ROOT_3 * hex_side_length),
                 Vector2::new(-hex_side_length, 0.0))
            },
            GridDirection { q:  0, r: 1 } => {
                (Vector2::new(hex_side_length / 2.0, -HALF_ROOT_3 * hex_side_length),
                 Vector2::new(-hex_side_length / 2.0, -HALF_ROOT_3 * hex_side_length))
            },
            GridDirection { q:  1, r: 0 } => {
                (Vector2::new(hex_side_length, 0.0),
                 Vector2::new(hex_side_length / 2.0, -HALF_ROOT_3 * hex_side_length))
            },
            GridDirection { q: 1, r: -1 } => {
                (Vector2::new(hex_side_length / 2.0, HALF_ROOT_3 * hex_side_length),
                 Vector2::new(hex_side_length, 0.0))
            },
            GridDirection { q: 0, r: -1 } => {
                (Vector2::new(-hex_side_length / 2.0, HALF_ROOT_3 * hex_side_length),
                 Vector2::new(hex_side_length / 2.0, HALF_ROOT_3 * hex_side_length))
            },
            _ => {
                panic!("Invalid GridDirection specified: {:?}", self);
            }
        }
    }
}

#[derive(Copy, Clone, Deserialize)]
pub struct GridIndex {
    // Axial coordinates
    q: i64,
    r: i64
}

impl GridIndex {
    pub fn neighbor(&self, dir: GridDirection) -> GridIndex {
        GridIndex {
            q: self.q + dir.q,
            r: self.r + dir.r,
        }
    }

    // Cartesian coordinates for edge defined in direction of dir.
    pub fn edge_line_segment(&self, dir: GridDirection, hex_side_length: f64) -> Segment<f64> {
        // Converts Axial coords into Cartesian 2D coords. The Cartesian coordinates correspond to
        // the hex cell's center.
        let axial_to_cartesian = Matrix2::from_rows(&[
            RowVector2::new(1.5 * hex_side_length, 0.0),
            RowVector2::new(-HALF_ROOT_3 * hex_side_length, -2.0 * HALF_ROOT_3 * hex_side_length)]);

        let self_center_cartesian = axial_to_cartesian * Vector2::new(self.q as f64, self.r as f64);
        let (start_offset, end_offset) = dir.cartesian_offsets(hex_side_length);

        let start_point = Point::from(self_center_cartesian + start_offset);
        let end_point   = Point::from(self_center_cartesian + end_offset);
        Segment::new(start_point, end_point)
    }
}
