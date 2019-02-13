use liquidfun::box2d::dynamics::{
    body::Body,
    fixture::Fixture
};
use entities::Entity;
use slab::Slab;
use std::{
    cell::RefCell,
    rc::Rc,
};

static HIGHEST_BIT_USIZE: usize = 1 << 63;

pub trait DataSetter {
    fn get_data(&self) -> usize;
    fn set_data(&self, data: usize);
}

impl DataSetter for Body {
    fn get_data(&self) -> usize {
       self.get_user_data()
    }

    fn set_data(&self, data: usize) {
        self.set_user_data(data);
    }
}

impl DataSetter for Fixture {
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

    pub fn resolve(&self, encoded: usize) -> Option<Entity> {
        self.raw.borrow().resolve(encoded).cloned()
    }

    pub fn register<UserData: DataSetter>(&mut self, entity: Entity, user_data_owner: &UserData) {
        self.raw.borrow_mut().register(entity, user_data_owner);
    }

    pub fn unregister<UserData: DataSetter>(&mut self, user_data_owner: &UserData) {
        self.raw.borrow_mut().unregister(user_data_owner);
    }
}

struct RawEntityRegistrar {
    registrar: Slab<Entity>,
}

impl RawEntityRegistrar {
    pub fn new() -> RawEntityRegistrar {
        RawEntityRegistrar {
            registrar: Slab::new(),
        }
    }

    pub fn resolve(&self, encoded: usize) -> Option<&Entity> {
        Self::decode(encoded).and_then(|decoded_idx| self.registrar.get(decoded_idx))
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
        let user_data = Self::encode(self.registrar.insert(entity));
        user_data_owner.set_data(user_data);
    }

    pub fn unregister<UserData: DataSetter>(&mut self, user_data_owner: &UserData) {
        if let Some(idx) = Self::decode(user_data_owner.get_data()) {
            self.registrar.remove(idx);
        }
    }
}

pub struct RegisteredBody {
    pub data_setter: Body,
    pub registrar: EntityRegistrar,
    pub entity: Option<Entity>,
}

impl RegisteredBody {
    pub fn new(body: Body, registrar: EntityRegistrar, entity: Option<Entity>) -> RegisteredBody {
        let mut registered = RegisteredBody {
            data_setter: body,
            registrar,
            entity
        };

        if let Some(entity) = registered.entity {
            registered.registrar.register(entity, &registered.data_setter);
        }

        registered
    }

    pub fn register(&mut self, entity: Entity) {
        if let None = self.entity {
            self.registrar.register(entity, &self.data_setter);
            self.entity = Some(entity);
        }
    }
}

impl Drop for RegisteredBody {
    fn drop(&mut self) {
        self.registrar.unregister(&self.data_setter);

        let mut world = self.data_setter.get_world();
        world.destroy_body(&mut self.data_setter);
    }
}

pub struct RegisteredFixture {
    pub data_setter: Fixture,
    pub registrar: EntityRegistrar,
    pub entity: Option<Entity>,
}

impl RegisteredFixture {
    pub fn new(fixture: Fixture, registrar: EntityRegistrar, entity: Option<Entity>) -> RegisteredFixture {
        let mut registered = RegisteredFixture {
            data_setter: fixture,
            registrar,
            entity
        };

        if let Some(entity) = registered.entity {
            registered.registrar.register(entity, &registered.data_setter);
        }

        registered
    }

    pub fn register(&mut self, entity: Entity) {
        if let None = self.entity {
            self.registrar.register(entity, &self.data_setter);
            self.entity = Some(entity);
        }
    }
}

impl Drop for RegisteredFixture {
    fn drop(&mut self) {
        self.registrar.unregister(&self.data_setter);

        let body = self.data_setter.get_body();
        body.destroy_fixture(&mut self.data_setter);
    }
}
