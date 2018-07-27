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
use std::{
    cell::RefCell,
    rc::Rc,
};
use world;

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
                        (EntityType::PLAYER, EntityType::GROUND) => {
                            let player: &mut world::Player = entity1.resolve();
                            player.touch_ground();
                        },
                        _ => {}
                    }
                },
                _ => {}
            }
        }
        self.contacts.clear();
    }
}

impl ContactListener for PhysicsContactListener {
    fn begin_fixture_fixture(&mut self, contact: Contact) {
        for contact in contact.iter() {
            let user_data_a = contact.get_fixture_a().get_body().get_user_data();
            let user_data_b = contact.get_fixture_b().get_body().get_user_data();
            // Sort each pair for canonicalization.
            let contact_data = if user_data_a < user_data_b {
                (user_data_a, user_data_b)
            } else {
                (user_data_b, user_data_a)
            };
            self.contacts.push(contact_data);
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

