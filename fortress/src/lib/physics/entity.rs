use liquidfun;
use physics::EntityType;
use std::{
    self,
    collections::HashMap,
};

static HIGHEST_BIT_USIZE: usize = 1 << 63;

struct Void {
}

pub struct Entity {
    etype: EntityType,
    data: *const Void,
}

impl Entity {
    fn new<T>(etype: EntityType, t: &T) -> Entity {
        unsafe {
            Entity {
                etype,
                data: std::mem::transmute(t)
            }
        }
    }

    pub fn etype(&self) -> &EntityType {
        &self.etype
    }

    pub fn resolve<T>(&self) -> &mut T {
        unsafe {
            std::mem::transmute(self.data)
        }
    }
}

pub struct EntityRegistrar {
    registrar: HashMap<usize, Entity>,
    registration_counter: usize
}

impl EntityRegistrar {
    pub fn new() -> EntityRegistrar {
        EntityRegistrar {
            registrar: HashMap::new(),
            registration_counter: 0
        }
    }

    pub fn register<T>(&mut self, t: &T, etype: EntityType, body: &liquidfun::box2d::dynamics::body::Body) {
        self.registration_counter += 1;

        let entity = Entity::new(etype, t);
        self.registrar.insert(self.registration_counter, entity);
        body.set_user_data(Self::encode(self.registration_counter));
    }

    pub fn unregister(&mut self, body: &mut liquidfun::box2d::dynamics::body::Body) {
        if let Some(idx) = Self::decode(body.get_user_data()) {
            self.registrar.remove(&idx);
        }
    }

    pub fn resolve(&self, encoded: usize) -> Option<&Entity> {
       Self::decode(encoded).and_then(|decoded_idx| self.registrar.get(&decoded_idx))
    }

    fn encode(val: usize) -> usize {
        val | HIGHEST_BIT_USIZE
    }

    fn decode(val: usize) -> Option<usize> {
        match val {
            any if any & HIGHEST_BIT_USIZE == 0 => None,
            any =>  {
                Some(any & !HIGHEST_BIT_USIZE)
            }
        }
    }
}