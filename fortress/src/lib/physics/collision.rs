use crate::entities::Entity;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Contact {
    Started(Entity, Entity),
    Stopped(Entity, Entity)
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum ProximityType {
    Intersecting,
    WithinMargin,
    Disjoint,
}

impl ProximityType {
    pub fn from(proximity: ncollide2d::query::Proximity) -> ProximityType {
        match proximity {
            ncollide2d::query::Proximity::Intersecting => ProximityType::Intersecting,
            ncollide2d::query::Proximity::WithinMargin => ProximityType::WithinMargin,
            ncollide2d::query::Proximity::Disjoint => ProximityType::Disjoint,
        }
    }

    pub fn basically_touching(self) -> bool {
        match self {
            ProximityType::Intersecting | ProximityType::WithinMargin => true,
            _ => false
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Proximity {
    pub entity1: Entity,
    pub entity2: Entity,
    pub prev_type: ProximityType,
    pub curr_type: ProximityType,
}
