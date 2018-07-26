use super::settings::*;

/// A 2D column vector.
#[repr(C)]
#[derive(Debug, PartialEq, Default, Copy, Clone)]
pub struct Vec2 {
    pub x: Float32,
    pub y: Float32,
}

impl Vec2 {
	/// Construct using coordinates.
	pub fn new(x: Float32, y: Float32) -> Vec2 {
		Vec2 {x: x, y: y}
	}

	/// Set this vector to some specified coordinates.
	pub fn set(&mut self, x: Float32, y: Float32) {
		self.x = x;
		self.y = y;
	}

	/// Construct a vector with all zero coordinates.
	pub fn zero() -> Vec2 {
		Vec2::default()
	}
}

#[allow(non_snake_case)]
/// Construct using coordinates.
#[inline] pub fn Vec2(x: Float32, y: Float32) -> Vec2 { Vec2 { x: x, y: y } }

#[repr(C)]
#[derive(Debug, PartialEq, Default, Copy, Clone)]
pub struct Rot {
    pub sin: Float32,
    pub cos: Float32,
}

#[repr(C)]
#[derive(Debug, PartialEq, Default, Copy, Clone)]
pub struct Transform {
    pub pos: Vec2,
	pub rot: Rot,
}
