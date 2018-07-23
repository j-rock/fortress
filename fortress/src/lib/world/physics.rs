use app::StatusOr;
use dimensions::time::DeltaTime;
use file::{
    ConfigWatcher,
    SimpleConfigManager,
};
use liquidfun::box2d::{
    common::math::Vec2,
    dynamics::world::World,
    dynamics::world::WrappedWorld,
};

#[derive(Deserialize)]
struct SimulationConfig {
    velocity_iterations: i32,
    position_iterations: i32,
    gravity_x: f32,
    gravity_y: f32,
}

pub struct PhysicsSimulation {
    wrapped_world: WrappedWorld,
    config: SimpleConfigManager<SimulationConfig>
}

impl PhysicsSimulation {
    pub fn new(config_watcher: &mut ConfigWatcher) -> StatusOr<PhysicsSimulation> {
        let config = SimpleConfigManager::<SimulationConfig>::new(config_watcher, "physics_sim.conf")?;
        let gravity = {
            let config_data = config.get();
            Vec2::new(config_data.gravity_x, config_data.gravity_y)
        };
        let wrapped_world = WrappedWorld::new(&gravity);
        Ok(PhysicsSimulation {
            wrapped_world,
            config
        })
    }

    pub fn update(&mut self, dt: DeltaTime) {
        self.config.update();
        let config = self.config.get();
        let gravity = Vec2::new(config.gravity_x, config.gravity_y);
        self.wrapped_world.world.set_gravity(&gravity);
        self.wrapped_world.world.step(dt.as_f32_seconds(), config.velocity_iterations, config.position_iterations);
    }

    pub fn get_world_mut(&mut self) -> &mut World {
        &mut self.wrapped_world.world
    }
}