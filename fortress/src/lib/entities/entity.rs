use crate::entities::EntityType;

struct Void {}

#[derive(Copy, Clone, Debug)]
pub struct Entity {
    etype: EntityType,
    data: *const Void,
}

impl Entity {
    pub fn new<T>(etype: EntityType, t: *const T) -> Entity {
        Entity {
            etype,
            data: t as *const Void
        }
    }

    pub fn etype(&self) -> EntityType {
        self.etype
    }

    #[allow(clippy::mut_from_ref)]
    pub fn resolve<T>(&self) -> &mut T {
        unsafe {
            &mut *(self.data as *mut T)
        }
    }
}