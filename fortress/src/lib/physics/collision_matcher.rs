use audio::AudioPlayer;
use entity::{
    Entity,
    EntityType,
};

pub enum CollisionMatcher {
    MatchOne(Box<Fn(EntityType) -> bool>, Box<Fn(&AudioPlayer, Entity)>),
    MatchTwo(Box<Fn(EntityType) -> bool>, Box<Fn(EntityType) -> bool>, Box<Fn(&AudioPlayer, Entity,Entity)>)
}

impl CollisionMatcher {
    pub fn match_one(etype: EntityType, closure: Box<Fn(&AudioPlayer, Entity)>) -> CollisionMatcher {
        Self::fuzzy_match_one(Box::new(move |arg| { arg == etype}), closure)
    }

    pub fn match_two(etype1: EntityType, etype2: EntityType, closure: Box<Fn(&AudioPlayer, Entity, Entity)>) -> CollisionMatcher {
        let pred1 = Box::new(move |arg| { arg == etype1});
        let pred2 = Box::new(move |arg| { arg == etype2});
        CollisionMatcher::MatchTwo(pred1, pred2, closure)
    }

    pub fn fuzzy_match_one(predicate: Box<Fn(EntityType) -> bool>, closure: Box<Fn(&AudioPlayer, Entity)>) -> CollisionMatcher {
        CollisionMatcher::MatchOne(predicate, closure)
    }

    pub fn fuzzy_match_two(pred1: Box<Fn(EntityType) -> bool>,
                           pred2: Box<Fn(EntityType) -> bool>,
                           closure: Box<Fn(&AudioPlayer, Entity, Entity)>) -> CollisionMatcher {
        CollisionMatcher::MatchTwo(pred1, pred2, closure)
    }

    pub fn try_apply(&self, audio: &AudioPlayer, entity1: Entity, entity2: Entity) {
        match self {
            CollisionMatcher::MatchOne(ref predicate, ref closure) => {
                if predicate(entity1.etype()) {
                    closure(audio, entity1);
                }
                if predicate(entity2.etype()) {
                    closure(audio, entity2);
                }
            },
            CollisionMatcher::MatchTwo(ref pred1, ref pred2, ref closure) => {
                if pred1(entity1.etype()) && pred2(entity2.etype()) {
                    closure(audio, entity1, entity2);
                } else if pred1(entity2.etype()) && pred2(entity1.etype()) {
                    closure(audio, entity2, entity1);
                }
            },
        }
    }
}