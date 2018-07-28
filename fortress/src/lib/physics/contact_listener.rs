use entity::{
    EntityType,
    EntityRegistrar,
};
use liquidfun::box2d::{
    collision::Manifold,
    common::{
        settings::Int32,
    },
    dynamics::{
        contact::Contact,
        fixture::Fixture,
        world_callbacks::{
            ContactImpulse,
            ContactListener,
        },
    },
    particle::particle_system::{
        ParticleBodyContact,
        ParticleContact,
        ParticleSystem
    },
};
use player;
use std::{
    cell::RefCell,
    rc::Rc,
};

pub struct PhysicsContactListener {
    contacts: Vec<(usize, usize)>
}

impl PhysicsContactListener {
    pub fn new() -> PhysicsContactListener {
        PhysicsContactListener {
            contacts: vec!(),
        }
    }

    pub fn process_contacts(&mut self, registrar: &Rc<RefCell<EntityRegistrar>>) {
        for (user_data1, user_data2) in self.contacts.iter() {
            match (registrar.borrow().resolve(*user_data1), registrar.borrow().resolve(*user_data2)) {
                (Some(entity1), Some(entity2)) => {
                    match (entity1.etype(), entity2.etype()) {
                        (EntityType::Ground, EntityType::PlayerFootSensor)  => {
                            let player: &mut player::Player = entity2.resolve();
                            player.make_foot_contact();
                        },
                        _ => {}
                    }
                },
                _ => {}
            }
        }
        self.contacts.clear();
    }

    fn get_user_data(fixture: &Fixture) -> Option<usize> {
        let fixture_user_data = fixture.get_user_data();
        if fixture_user_data != 0 {
            return Some(fixture_user_data);
        }

        match fixture.get_body().get_user_data() {
            0 => None,
            any => Some(any)
        }
    }
}

impl ContactListener for PhysicsContactListener {
    fn begin_fixture_fixture(&mut self, contact: Contact) {
        for contact in contact.iter() {
            let user_data_a = Self::get_user_data(&contact.get_fixture_a());
            let user_data_b = Self::get_user_data(&contact.get_fixture_b());
            match (user_data_a, user_data_b) {
                (Some(data_a), Some(data_b)) => {
                    // Sort each pair for canonicalization.
                    let contact_data = if data_a < data_b {
                        (data_a, data_b)
                    } else {
                        (data_b, data_a)
                    };
                    self.contacts.push(contact_data);
                },
                _ => {}
           }
        }
    }

    fn end_fixture_fixture(&mut self, _contact: Contact) {}
    fn begin_particle_fixture(&mut self, _particle_system: ParticleSystem, _particle_body_contact: &ParticleBodyContact) {}
    fn end_particle_fixture(&mut self, _fixture: Fixture, _particle_system: ParticleSystem, _index: Int32) {}
    fn begin_particle_particle(&mut self, _particle_system: ParticleSystem, _particle_contact: ParticleContact) {}
    fn end_particle_particle(&mut self, _particle_system: ParticleSystem, _index_a: Int32, _index_b: Int32) {}
    fn pre_solve(&mut self, _contact: Contact, _old_manifold: &Manifold) {}
    fn post_solve(&mut self, _contact: Contact, _impulse: &ContactImpulse) {}
}

