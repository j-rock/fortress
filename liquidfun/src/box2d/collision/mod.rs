pub mod shapes;

use std::mem;
use super::common::settings::*;
use super::common::math::*;

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum ContactFeatureType
{
    Vertex = 0,
    Face = 1,
}

/// The features that intersect to form the contact point
/// This must be 4 bytes or less.
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct ContactFeature
{
    /// Feature index on shapeA
    pub index_a: UInt8,
    /// Feature index on shapeB
    pub index_b: UInt8,
    /// The feature type on shapeA
    pub type_a: UInt8,
    /// The feature type on shapeB
    pub type_b: UInt8,
}

/// Contact ids to facilitate warm starting.
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct ContactId(u32);

impl ContactId {
    pub fn feature(self) -> ContactFeature {
        unsafe { mem::transmute(self.0) }
    }

    pub fn key(self) -> u32 {
        self.0
    }
}

/// A manifold point is a contact point belonging to a contact
/// manifold. It holds details related to the geometry and dynamics
/// of the contact points.
/// The local point usage depends on the manifold type:
/// -e_circles: the local center of circleB
/// -e_faceA: the local center of cirlceB or the clip point of polygonB
/// -e_faceB: the clip point of polygonA
/// This structure is stored across time steps, so we keep it small.
/// Note: the impulses are used for internal caching and may not
/// provide reliable contact forces, especially for high speed collisions.
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct ManifoldPoint
{
    /// usage depends on manifold type
    pub local_point: Vec2,
    /// the non-penetration impulse
    pub normal_impulse: Float32,
    /// the friction impulse
    pub tangent_impulse: Float32,
    /// uniquely identifies a contact point between two shapes
    pub id: ContactId,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum ManifoldType
{
    Circles,
    FaceA,
    FaceB,
}

/// A manifold for two touching convex shapes.
/// Box2D supports multiple types of contact:
/// - clip point versus plane with radius
/// - point versus point with radius (circles)
/// The local point usage depends on the manifold type:
/// -e_circles: the local center of circleA
/// -e_faceA: the center of faceA
/// -e_faceB: the center of faceB
/// Similarly the local normal usage:
/// -e_circles: not used
/// -e_faceA: the normal on polygonA
/// -e_faceB: the normal on polygonB
/// We store contacts in this way so that position correction can
/// account for movement, which is critical for continuous physics.
/// All contact scenarios must be expressed in one of these types.
/// This structure is stored across time steps, so we keep it small.
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Manifold
{
    /// the points of contact
    pub points: [ManifoldPoint; MAX_MANIFOLD_POINTS],
    /// not use for Type::e_points
    pub local_normal: Vec2,
    /// usage depends on manifold type
    pub local_point: Vec2,
    pub manifold_type: ManifoldType,
    /// the number of manifold points
    pub point_count: Int32,
}

/// This is used to compute the current state of a contact manifold.
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct WorldManifold
{
    /// world vector pointing from A to B
    pub normal: Vec2,
    /// world contact point (point of intersection)
    pub points: [Vec2; MAX_MANIFOLD_POINTS],
    /// a negative value indicates overlap, in meters
    pub separations: [Float32; MAX_MANIFOLD_POINTS],
}

/// This is used for determining the state of contact points.
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum PointState
{
    /// point does not exist
    NullState,
    /// point was added in the update
    AddState,
    /// point persisted across the update
    PersistState,
    /// point was removed in the update
    RemoveState,
}

/// Used for computing contact manifolds.
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct ClipVertex
{
	pub v: Vec2,
	pub id: ContactId,
}

/// Ray-cast input data. The ray extends from p1 to p1 + maxFraction * (p2 - p1).
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct RayCastInput
{
	pub p1: Vec2,
    pub p2: Vec2,
	pub max_fraction: Float32,
}

/// Ray-cast output data. The ray hits at p1 + fraction * (p2 - p1), where p1 and p2
/// come from b2RayCastInput.
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct RayCastOutput
{
	pub normal: Vec2,
	pub fraction: Float32,
}

/// An axis aligned bounding box.
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct AABB
{
    /// the lower vertex
    pub lower_bound: Vec2,
    /// the upper vertex
    pub upper_bound: Vec2,
}
