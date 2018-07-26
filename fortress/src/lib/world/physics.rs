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

        Ok(PhysicsSimulation {
            config,
            wrapped_world,
            contact_listener,
            _contact_listener_glue: contact_listener_glue,
        })
    }

    pub fn update(&mut self, dt: DeltaTime) {
        self.config.update();
        let config = self.config.get();
        let gravity = Vec2::new(config.gravity_x, config.gravity_y);
        self.wrapped_world.world.set_gravity(&gravity);

        self.wrapped_world.world.step(dt.as_f32_seconds(), config.velocity_iterations, config.position_iterations);
        self.contact_listener.process_contacts();
    }

    pub fn get_world_mut(&mut self) -> &mut World {
        &mut self.wrapped_world.world
    }
}

struct PhysicsContactListener {
}

impl PhysicsContactListener {
    pub fn new() -> PhysicsContactListener {
        PhysicsContactListener { }
    }

    pub fn process_contacts(&mut self) {
    }
}

impl ContactListener for PhysicsContactListener {
    fn begin_fixture_fixture(&mut self, _contact: Contact) {}
    fn end_fixture_fixture(&mut self, _contact: Contact) {}
    fn begin_particle_fixture(&mut self, _particle_system: ParticleSystem, _particle_body_contact: &ParticleBodyContact) {}
    fn end_particle_fixture(&mut self, _fixture: Fixture, _particle_system: ParticleSystem, _index: Int32) {}
    fn begin_particle_particle(&mut self, _particle_system: ParticleSystem, _particle_contact: ParticleContact) {}
    fn end_particle_particle(&mut self, _particle_system: ParticleSystem, _index_a: Int32, _index_b: Int32) {}
    fn pre_solve(&mut self, _contact: Contact, _old_manifold: &Manifold) {}
    fn post_solve(&mut self, _contact: Contact, _impulse: &ContactImpulse) {}
}

