use crate::{
    physics::{
        Contact,
        Proximity,
    },
    world::WorldView,
};

pub struct PhysicsMatcher<T> {
    closure: Box<dyn Fn(T, &mut WorldView)>,
}

impl <T> PhysicsMatcher<T> {
    pub fn new(closure: Box<Fn(T, &mut WorldView)>) -> PhysicsMatcher<T> {
        PhysicsMatcher {
            closure
        }
    }

    pub fn try_apply(&self, data: T, world: &mut WorldView) {
        (*self.closure)(data, world);
    }
}

pub type ProximityMatcher = PhysicsMatcher<Proximity>;
pub type ContactMatcher = PhysicsMatcher<Contact>;
