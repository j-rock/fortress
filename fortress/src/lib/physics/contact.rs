use entity::Entity;
use std;

pub struct Contact<A, B> {
    _phantom_a: std::marker::PhantomData<A>,
    _phantom_b: std::marker::PhantomData<B>,
}

impl <A,B> Contact<A, B> {
    pub fn new() -> Contact<A,B> {
        Contact {
            _phantom_a: std::marker::PhantomData,
            _phantom_b: std::marker::PhantomData,
        }
    }
}

pub trait BeginFixtureFixture {
    fn begin_fixture_fixture(&self, entity_a: &Entity, entity_b: &Entity);
}

/*
impl <T> BeginFixtureFixture for Interact<PlayerFoot, T> {
    pub fn begin_fixture_fixture(&self, entity_a: &Entity, _entity_b: &Entity) {
        let player: &mut world::Player = self.first.resolve();
        player.touching_ground();
    }
}

match (e1.etype(), e2.etype()) {
    (PlayerEntity(PlayerFoot), _) => {
        Interact::new(PlayerFoot, (), e1, e2).begin_contact();
    }
}
*/
