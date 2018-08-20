use entity::{
    Entity,
    EntityType,
};

pub enum CollisionMatcher {
    MatchOne(Box<Fn(EntityType) -> bool>, Box<Fn(Entity)>),
    MatchTwo(EntityType, EntityType, Box<Fn(Entity, Entity)>),
}

impl CollisionMatcher {
    pub fn match_one(etype: EntityType, closure: Box<Fn(Entity)>) -> CollisionMatcher {
        Self::fuzzy_match_one(Box::new(move |arg| { arg == etype}), closure)
    }

    pub fn match_two(etype1: EntityType, etype2: EntityType, closure: Box<Fn(Entity, Entity)>) -> CollisionMatcher {
        CollisionMatcher::MatchTwo(etype1, etype2, closure)
    }

    pub fn fuzzy_match_one(predicate: Box<Fn(EntityType) -> bool>, closure: Box<Fn(Entity)>) -> CollisionMatcher {
        CollisionMatcher::MatchOne(predicate, closure)
    }

    pub fn try_apply(&self, entity1: Entity, entity2: Entity) {
        match self {
            CollisionMatcher::MatchOne(ref predicate, ref closure) => {
                if predicate(entity1.etype()) {
                    closure(entity1);
                }
                if predicate(entity2.etype()) {
                    closure(entity2);
                }
            },
            CollisionMatcher::MatchTwo(ref etype1, ref etype2, ref closure) => {
                if *etype1 == entity1.etype() && *etype2 == entity2.etype() {
                    closure(entity1, entity2);
                } else if *etype1 == entity2.etype() && *etype2 == entity1.etype() {
                    closure(entity2, entity1);
                }
            },
        }
    }
}