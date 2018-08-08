use entity::EntityType;
use std;

struct Void {
}

#[derive(Copy, Clone)]
pub struct Entity {
    etype: EntityType,
    data: *const Void,
}

impl Entity {
    pub fn new<T>(etype: EntityType, t: *const T) -> Entity {
        unsafe {
            Entity {
                etype,
                data: std::mem::transmute(t)
            }
        }
    }

    pub fn etype(&self) -> &EntityType {
        &self.etype
    }

    pub fn resolve<T>(&self) -> &mut T {
        unsafe {
            &mut *(self.data as *mut T)
        }
    }
}
