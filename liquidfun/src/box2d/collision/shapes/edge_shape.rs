use super::shape::*;
use super::super::super::common::math::*;

enum B2EdgeShape {}

extern {
    fn b2EdgeShape_New() -> *mut B2EdgeShape;
    fn b2EdgeShape_Delete(ptr: *mut B2EdgeShape);
    fn b2EdgeShape_Upcast(ptr: *mut B2EdgeShape) -> *mut B2Shape;
    fn b2EdgeShape_Set(ptr: *mut B2EdgeShape, v1: *const Vec2, v2: *const Vec2);
    fn b2EdgeShape_Set0(ptr: *mut B2EdgeShape, v: *const Vec2);
    fn b2EdgeShape_Set3(ptr: *mut B2EdgeShape, v: *const Vec2);
}

/// A line segment (edge) shape. These can be connected in chains or loops
/// to other edge shapes. The connectivity information is used to ensure
/// correct contact normals.
pub struct EdgeShape {
    ptr: *mut B2EdgeShape,
    owned: bool,
}

/// Cast a EdgeShape from a B2Shape.
pub fn from_shape(ptr: *mut B2Shape) -> EdgeShape {
    EdgeShape { ptr: ptr as *mut B2EdgeShape, owned: false }
}

impl Shape for EdgeShape {
    // Is the up-cast even necessary? Can't we just use self.ptr directly?
    fn handle(&self) -> *mut B2Shape {
        unsafe {
            b2EdgeShape_Upcast(self.ptr)
        }
    }
}

impl EdgeShape {

    /// Create a new EdgeShape.
    pub fn new() -> EdgeShape {
        unsafe {
            EdgeShape { ptr: b2EdgeShape_New(), owned: true }
        }
    }

    pub fn set(&mut self, v1: &Vec2, v2: &Vec2) {
        unsafe {
            b2EdgeShape_Set(self.ptr, v1, v2);
        }
    }

    pub fn set0(&mut self, v: Option<&Vec2>) {
        unsafe {
            b2EdgeShape_Set0(self.ptr,
                if let Some(v) = v { v } else { ::std::ptr::null() });
        }
    }

    pub fn set3(&mut self, v: Option<&Vec2>) {
        unsafe {
            b2EdgeShape_Set3(self.ptr,
                if let Some(v) = v { v } else { ::std::ptr::null() });
        }
    }

}

impl Drop for EdgeShape {
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                b2EdgeShape_Delete(self.ptr);
            }
        }
    }
}
