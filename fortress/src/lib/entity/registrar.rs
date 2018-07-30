use liquidfun;
use entity::{
    Entity,
    EntityType,
};
use std::collections::HashMap;

pub trait DataSetter {
    fn get_data(&self) -> usize;
    fn set_data(&self, data: usize);
}

impl DataSetter for liquidfun::box2d::dynamics::body::Body {
    fn get_data(&self) -> usize {
       self.get_user_data()
    }

    fn set_data(&self, data: usize) {
        self.set_user_data(data);
    }
}

impl DataSetter for liquidfun::box2d::dynamics::fixture::Fixture {
    fn get_data(&self) -> usize {
        self.get_user_data()
    }

    fn set_data(&self, data: usize) {
        self.set_user_data(data);
    }
}

static HIGHEST_BIT_USIZE: usize = 1 << 63;

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

    pub fn register<Data, UserData: DataSetter>(&mut self, data: *const Data, etype: EntityType, user_data_owner: &UserData) {
        self.registration_counter += 1;

        let entity = Entity::new(etype, data);
        self.registrar.insert(self.registration_counter, entity);
        let user_data = Self::encode(self.registration_counter);
        user_data_owner.set_data(user_data);
    }

    pub fn unregister<UserData: DataSetter>(&mut self, user_data_owner: &UserData) {
        if let Some(idx) = Self::decode(user_data_owner.get_data()) {
            self.registrar.remove(&idx);
        }
    }
}

pub struct Registered<T> {
    pub data: T,
    is_registered: bool,
    etype: EntityType,
}

impl <T> Registered<T> {
    pub fn new(data: T, etype: EntityType) -> Registered<T> {
        Registered {
            data,
            is_registered: false,
            etype,
        }
    }
}

impl <T: DataSetter> Registered<T> {
    pub fn register<Data>(&mut self, registrar: &mut EntityRegistrar, data: *const Data) {
        if !self.is_registered {
            self.is_registered = true;
            registrar.register(data, self.etype, &self.data);
        }
    }

    pub fn unregister(&self, registrar: &mut EntityRegistrar) {
        if self.is_registered {
            registrar.unregister(&self.data);
        }
    }
}
