use std::f32;
use libc::*;

pub type Int8 = c_char;
pub type Int16 = c_short;
pub type Int32 = c_int;
pub type UInt8 = c_uchar;
pub type UInt16 = c_ushort;
pub type UInt32 = c_uint;
pub type Float32 = c_float;
pub type Float64 = c_double;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct B2Version {
    pub major: Int32,
    pub minor: Int32,
    pub revision: Int32,
}

pub const MAX_MANIFOLD_POINTS: usize = 2;
pub const MAX_POLYGON_VERTICES: usize = 8;
pub const AABB_EXTENSION: Float32 = 0.1;
pub const AABB_MULTIPLIER: Float32 = 2.0;
pub const LINEAR_SLOP: Float32 = 0.005;
pub const ANGULAR_SLOP: Float32 = (2.0 / 180.0 * f32::consts::PI);
pub const POLYGON_RADIUS: Float32 = (2.0 * LINEAR_SLOP);
pub const MAX_SUB_STEPS: usize = 8;
pub const MAX_TOI_CONTACTS: usize = 32;
pub const VELOCITY_THRESHOLD: Float32 = 1.0;
pub const MAX_LINEAR_CORRECTION: Float32 = 0.2;
pub const MAX_ANGULAR_CORRECTION: Float32 = (8.0 / 180.0 * f32::consts::PI);
pub const MAX_TRANSLATION: Float32 = 2.0;
pub const MAX_TRANSLATION_SQUARED: Float32 = (MAX_TRANSLATION * MAX_TRANSLATION);
pub const MAX_ROTATION: Float32 = (0.5 * f32::consts::PI);
pub const MAX_ROTATION_SQUARED: Float32 = (MAX_ROTATION * MAX_ROTATION);
pub const BAUMGARTE: Float32 = 0.2;
pub const TOI_BAUGARTE: Float32 = 0.75;
pub const PARTICLE_STRIDE: Float32 = 0.75;
pub const MIN_PARTICLE_WEIGHT: Float32 = 1.0;
pub const MAX_PARTICLE_PRESSURE: Float32 = 0.25;
pub const MAX_PARTICLE_FORCE: Float32 = 0.5;
pub const MAX_TRIAD_DISTANCE: usize = 2;
pub const MAX_TRIAD_DISTANCE_SQUARED: usize = (MAX_TRIAD_DISTANCE * MAX_TRIAD_DISTANCE);
pub const MIN_PARTICLE_SYSTEM_BUFFER_CAPACITY: usize = 256;
pub const BARRIER_COLLISION_TIME: Float32 = 2.5;
pub const TIME_TO_SLEEP: Float32 = 0.5;
pub const LINEAR_SLEEP_TOLERANCE: Float32 = 0.01;
pub const ANGULAR_SLEEP_TOLERANCE: Float32 = (2.0 / 180.0 * f32::consts::PI);

extern {
    pub static b2_version: B2Version;
    pub static b2_liquidFunVersion: B2Version;
    pub static b2_liquidFunVersionString: *const c_char;
}
