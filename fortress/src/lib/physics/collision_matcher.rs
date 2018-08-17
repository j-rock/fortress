use entity::{
    Entity,
    EntityType,
};

pub enum CollisionMatcher {
    MatchOne(EntityType, Box<Fn(Entity)>),
    MatchTwo(EntityType, EntityType, Box<Fn(Entity, Entity)>),
}

impl CollisionMatcher {
    pub fn match_one(etype: EntityType, closure: Box<Fn(Entity)>) -> CollisionMatcher {
        CollisionMatcher::MatchOne(etype, closure)
    }

    pub fn match_two(etype1: EntityType, etype2: EntityType, closure: Box<Fn(Entity, Entity)>) -> CollisionMatcher {
        CollisionMatcher::MatchTwo(etype1, etype2, closure)
    }

    pub fn try_apply(&self, entity1: Entity, entity2: Entity) {
        match self {
            CollisionMatcher::MatchOne(ref etype, ref closure) => {
                if *etype == entity1.etype() {
                    closure(entity1);
                }
                if *etype == entity2.etype() {
                    closure(entity2);
                }
            },
            CollisionMatcher::MatchTwo(ref etype1, ref etype2, ref closure) => {
                if *etype1 == entity1.etype() && *etype2 == entity2.etype() {
                    closure(entity1, entity2);
                } else if *etype1 == entity2.etype() && *etype2 == entity1.etype() {
                    closure(entity2, entity1);
                }
            }
        }
    }
}