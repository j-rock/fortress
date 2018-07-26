//! A shape is used for collision detection. You can create a shape however you like.
//! Shapes used for simulation in b2World are created automatically when a b2Fixture
//! is created. Shapes may encapsulate a one or more child shapes.

use super::super::super::common::math::*;
use super::super::super::common::settings::*;

/// This holds the mass data computed for a shape.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct MassData
{
	/// The mass of the shape, usually in kilograms.
	pub mass: Float32,

	/// The position of the shape's centroid relative to the shape's origin.
	pub center: Vec2,

	/// The rotational inertia of the shape about the local origin.
	pub i: Float32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub enum Type
{
	Circle = 0,
	Edge = 1,
	Polygon = 2,
	Chain = 3,
	TypeCount = 4
}

pub enum B2Shape {}

pub trait Shape {
	fn handle(&self) -> *mut B2Shape;
}
