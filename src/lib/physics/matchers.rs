use crate::{
    entities::Entity,
    world::WorldView,
};
use ncollide2d::events::{
    ContactEvent,
    ProximityEvent,
};

pub struct PhysicsMatcher<T> {
    closure: Box<dyn Fn(Entity, Entity, &T, &mut WorldView)>,
}

impl <T> PhysicsMatcher<T> {
    pub fn new(closure: Box<Fn(Entity, Entity, &T, &mut WorldView)>) -> PhysicsMatcher<T> {
        PhysicsMatcher {
            closure
        }
    }

    pub fn try_apply(&self, entity1: Entity, entity2: Entity, data: &T, world: &mut WorldView) {
        (*self.closure)(entity1, entity2, data, world);
    }
}

pub type ProximityMatcher = PhysicsMatcher<ProximityEvent>;
pub type ContactMatcher = PhysicsMatcher<ContactEvent>;
