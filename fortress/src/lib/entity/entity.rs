use entity::EntityType;
use std;

struct Void {
}

pub struct Entity {
    etype: EntityType,
    data: *const Void,
}

impl Entity {
    pub fn new<T>(etype: EntityType, t: &T) -> Entity {
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
            std::mem::transmute(self.data)
        }
    }
}
