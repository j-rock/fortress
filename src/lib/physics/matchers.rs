use crate::{
    entities::Entity,
    world::WorldView,
};
use ncollide2d::events::{
    ContactEvent,
    ProximityEvent,
};

pub enum PhysicsMatcher<T> {
    MatchOne(Box<Fn(Entity, &T) -> bool>, Box<Fn(Entity, &mut WorldView)>),
    MatchTwo(Box<Fn(Entity, Entity, &T) -> bool>, Box<Fn(Entity, Entity, &mut WorldView)>)
}

impl <T> PhysicsMatcher<T> {
    pub fn try_apply<'a>(&self, entity1: Entity, entity2: Entity, data: &T, world: &mut WorldView<'a>) {
        match self {
            PhysicsMatcher::MatchOne(ref predicate, ref closure) => {
                if predicate(entity1, data) {
                    closure(entity1, world);
                }
                if predicate(entity2, data) {
                    closure(entity2, world);
                }
            },
            PhysicsMatcher::MatchTwo(ref predicate, ref closure) => {
                if predicate(entity1, entity2, data) {
                    closure(entity1, entity2, world);
                } else if predicate(entity2, entity1, data) {
                    closure(entity2, entity1, world);
                }
            }
        }
    }
}

pub type ProximityMatcher = PhysicsMatcher<ProximityEvent>;
pub type ContactMatcher = PhysicsMatcher<ContactEvent>;
