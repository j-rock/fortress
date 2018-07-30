use app::StatusOr;
use dimensions::time::DeltaTime;
use file::{
    ConfigWatcher,
    SimpleConfigManager,
};
use entity::EntityRegistrar;
use liquidfun::box2d::{
    common::math::Vec2,
    dynamics::{
        world::{
            World,
            WrappedWorld
        },
        world_callbacks::ContactListenerGlue
    }
};
use physics::PhysicsContactListener;

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
    contact_listener: Box<PhysicsContactListener>,
    contact_listener_glue: ContactListenerGlue,
    registrar: EntityRegistrar,
}

impl PhysicsSimulation {
    pub fn new(config_watcher: &mut ConfigWatcher) -> StatusOr<PhysicsSimulation> {
        let config = SimpleConfigManager::<SimulationConfig>::new(config_watcher, "physics_sim.conf")?;
        let gravity = {
            let config_data = config.get();
            Vec2::new(config_data.gravity_x, config_data.gravity_y)
        };

        let mut sim = PhysicsSimulation {
            config,
            wrapped_world: WrappedWorld::new(&gravity),
            contact_listener: Box::new(PhysicsContactListener::new()),
            contact_listener_glue: ContactListenerGlue::new(),
            registrar: EntityRegistrar::new(),
        };

        sim.wrapped_world.world.set_contact_listener(sim.contact_listener.as_mut(), &mut sim.contact_listener_glue);

        Ok(sim)
    }

    pub fn update(&mut self, dt: DeltaTime) {
        self.config.update();
        let config = self.config.get();
        let gravity = Vec2::new(config.gravity_x, config.gravity_y);
        self.wrapped_world.world.set_gravity(&gravity);
        self.wrapped_world.world.step(dt.as_f32_seconds(), config.velocity_iterations, config.position_iterations);
        self.contact_listener.process_contacts(&mut self.registrar);
    }

    pub fn get_world_mut(&mut self) -> &mut World {
        &mut self.wrapped_world.world
    }

    pub fn registrar_mut(&mut self) -> &mut EntityRegistrar {
        &mut self.registrar
    }
}

