use app::StatusOr;
use dimensions::time::DeltaTime;
use file::{
    ConfigWatcher,
    SimpleConfigManager,
};
use liquidfun::box2d::{
    collision::Manifold,
    common::{
        math::Vec2,
        settings::Int32,
    },
    dynamics::{
        contact::Contact,
        fixture::Fixture,
        world::{
            World,
            WrappedWorld
        },
        world_callbacks::{
            ContactImpulse,
            ContactListener,
            ContactListenerGlue,
        },
    },
    particle::particle_system::{
        ParticleBodyContact,
        ParticleContact,
        ParticleSystem
    },
};
use physics::{
    EntityRegistrar,
    EntityType,
};
use std::{
    cell::RefCell,
    rc::Rc,
};
use world;

#[derive(Deserialize)]
struct SimulationConfig {
    velocity_iterations: i32,
    position_iterations: i32,
    gravity_x: f32,
    gravity_y: f32,
}

pub struct PhysicsSimulation {
    config: SimpleConfigManager<SimulationConfig>,
    wrapped_world: WrappedWorld,
    contact_listener: PhysicsContactListener,
    _contact_listener_glue: ContactListenerGlue,
    registrar: Rc<RefCell<EntityRegistrar>>,
}

impl PhysicsSimulation {
    pub fn new(config_watcher: &mut ConfigWatcher) -> StatusOr<PhysicsSimulation> {
        let config = SimpleConfigManager::<SimulationConfig>::new(config_watcher, "physics_sim.conf")?;
        let gravity = {
            let config_data = config.get();
            Vec2::new(config_data.gravity_x, config_data.gravity_y)
        };

        let mut contact_listener = PhysicsContactListener::new();
        let mut contact_listener_glue = ContactListenerGlue::new();

        let mut wrapped_world = WrappedWorld::new(&gravity);
        wrapped_world.world.set_contact_listener(&mut contact_listener, &mut contact_listener_glue);

        let registrar = Rc::new(RefCell::new(EntityRegistrar::new()));

        Ok(PhysicsSimulation {
            config,
            wrapped_world,
            contact_listener,
            _contact_listener_glue: contact_listener_glue,
            registrar
        })
    }

    pub fn update(&mut self, dt: DeltaTime) {
        self.config.update();
        let config = self.config.get();
        let gravity = Vec2::new(config.gravity_x, config.gravity_y);
        self.wrapped_world.world.set_gravity(&gravity);

        self.contact_listener.clear_for_world_step();
        self.wrapped_world.world.step(dt.as_f32_seconds(), config.velocity_iterations, config.position_iterations);
        self.contact_listener.process_contacts(&self.registrar);
    }

    pub fn get_world_mut(&mut self) -> &mut World {
        &mut self.wrapped_world.world
    }

    pub fn registrar(&self) -> Rc<RefCell<EntityRegistrar>> {
        Rc::clone(&self.registrar)
    }
}

struct PhysicsContactListener {
    contacts: Vec<(usize, usize)>
}

impl PhysicsContactListener {
    pub fn new() -> PhysicsContactListener {
        PhysicsContactListener {
            contacts: vec!(),
        }
    }

    pub fn clear_for_world_step(&mut self) {
       self.contacts.clear();
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

