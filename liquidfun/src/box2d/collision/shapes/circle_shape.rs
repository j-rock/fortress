use super::shape::*;
use super::super::super::common::math::*;

enum B2CircleShape {}

extern {
    fn b2CircleShape_New(position: *const Vec2, radius: f32) -> *mut B2CircleShape;
    fn b2CircleShape_Delete(ptr: *mut B2CircleShape);
    fn b2CircleShape_Upcast(ptr: *mut B2CircleShape) -> *mut B2Shape;
}

pub struct CircleShape {
    ptr: *mut B2CircleShape,
}

impl Shape for CircleShape {
    fn handle(&self) -> *mut B2Shape {
        unsafe {
            b2CircleShape_Upcast(self.ptr)
        }
    }
}

impl CircleShape {
    /// Create a new CircleShape.
    pub fn new(position: Vec2, radius: f32) -> CircleShape {
        unsafe {
            CircleShape { ptr: b2CircleShape_New(&position, radius)}
        }
    }
}

impl Drop for CircleShape {
    fn drop(&mut self) {
        unsafe {
            b2CircleShape_Delete(self.ptr);
        }
    }
}
