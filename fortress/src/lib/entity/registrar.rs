use liquidfun;
use entity::Entity;
use std::{
    cell::RefCell,
    collections::HashMap,
    rc::Rc,
};

static HIGHEST_BIT_USIZE: usize = 1 << 63;

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

#[derive(Clone)]
pub struct EntityRegistrar {
    raw: Rc<RefCell<RawEntityRegistrar>>
}

impl EntityRegistrar {
    pub fn new() -> EntityRegistrar {
        EntityRegistrar {
            raw: Rc::new(RefCell::new(RawEntityRegistrar::new()))
        }
    }

    pub fn resolve(&self, encoded: usize) -> Option<&Entity> {
        self.raw.borrow_mut().resolve(encoded)
    }

    pub fn register<UserData: DataSetter>(&mut self, entity: Entity, user_data_owner: &UserData) {
        self.raw.borrow_mut().register(entity, user_data_owner);
    }

    pub fn unregister<UserData: DataSetter>(&mut self, user_data_owner: &UserData) {
        self.raw.borrow_mut().unregister(user_data_owner);
    }
}

struct RawEntityRegistrar {
    registrar: HashMap<usize, Entity>,
    registration_counter: usize
}

impl RawEntityRegistrar {
    pub fn new() -> RawEntityRegistrar {
        RawEntityRegistrar {
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
        if val & HIGHEST_BIT_USIZE == 0 {
            None
        } else {
            Some(val & !HIGHEST_BIT_USIZE)
        }
    }

    pub fn register<UserData: DataSetter>(&mut self, entity: Entity, user_data_owner: &UserData) {
        self.registration_counter += 1;
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

pub struct Registered<T: DataSetter> {
    pub data_setter: T,
    pub registrar: EntityRegistrar,
    pub entity: Option<Entity>,
}

impl <T: DataSetter> Registered<T> {
    pub fn new(data_setter: T, registrar: EntityRegistrar, entity: Option<Entity>) -> Registered<T> {
        if let Some(entity) = entity {
            registrar.register(entity, &data_setter);
        }

        Registered {
            data_setter,
            entity,
            registrar
        }
    }

    pub fn register(&mut self, entity: Entity) {
        if let None = self.entity {
            self.registrar.register(entity, &self.data_setter);
            self.entity = Some(entity);
        }
    }
}

impl <T: DataSetter> Drop for Registered<T> {
    fn drop(&mut self) {
        self.registrar.unregister(&self.data_setter);
    }
}
