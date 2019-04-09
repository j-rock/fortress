use crate::{
    audio::AudioPlayer,
    dimensions::time::DeltaTime,
    entities::Entity,
};
use ncollide2d::events::{
    ContactEvent,
    ProximityEvent,
};

pub enum PhysicsMatcher<T> {
    MatchOne(Box<Fn(Entity, &T) -> bool>, Box<Fn(Entity, &AudioPlayer, DeltaTime)>),
    MatchTwo(Box<Fn(Entity, Entity, &T) -> bool>, Box<Fn(Entity, Entity, &AudioPlayer, DeltaTime)>)
}

impl <T> PhysicsMatcher<T> {
    pub fn try_apply(&self, entity1: Entity, entity2: Entity, data: &T, audio: &AudioPlayer, dt: DeltaTime) {
        match self {
            PhysicsMatcher::MatchOne(ref predicate, ref closure) => {
                if predicate(entity1, data) {
                    closure(entity1, audio, dt);
                }
                if predicate(entity2, data) {
                    closure(entity2, audio, dt);
                }
            },
            PhysicsMatcher::MatchTwo(ref predicate, ref closure) => {
                if predicate(entity1, entity2, data) {
                    closure(entity1, entity2, audio, dt);
                } else if predicate(entity2, entity1, data) {
                    closure(entity2, entity1, audio, dt);
                }
            }
        }
    }
}

pub type ProximityMatcher = PhysicsMatcher<ProximityEvent>;
pub type ContactMatcher = PhysicsMatcher<ContactEvent>;
