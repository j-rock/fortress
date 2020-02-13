use crate::entities::{
    Entity,
    EntityId,
};
use std::collections::HashMap;

pub struct EntityRegistrar {
    registrar: HashMap<EntityId, Entity>,
}

impl Default for EntityRegistrar {
    fn default() -> Self {
        Self::new()
    }
}

impl EntityRegistrar {
    pub fn new() -> EntityRegistrar {
        EntityRegistrar {
            registrar: HashMap::new(),
        }
    }

    pub fn resolve(&self, id: EntityId) -> Option<Entity> {
        self.registrar.get(&id).cloned()
    }

    pub fn register(&mut self, id: EntityId, entity: Entity) {
        let maybe_preexisting = self.registrar.insert(id, entity);
        if maybe_preexisting.is_some() {
            panic!("This id has already been registered! {:?}", id);
        };
    }

    pub fn unregister(&mut self, id: EntityId) {
        self.registrar.remove(&id);
    }
}
