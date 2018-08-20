use app::StatusOr;
use control::{
    Controller,
    ControlEvent,
};
use dimensions::{
    LrDirection,
    time::DeltaTime
};
use file::{
    ConfigWatcher,
    SimpleConfigManager,
};
use glm;
use physics::PhysicsSimulation;
use wraith::{
    WraithConfig,
    WraithState,
    state::{
        WraithBody,
        WraithStateMachine,
        WraithUpright,
    }
};
use render::{
    BoxData,
    BoxRenderer,
};

pub struct Wraith {
    config_manager: SimpleConfigManager<WraithConfig>,
    wraith_state: WraithState,
    wraith_state_machine: Box<dyn WraithStateMachine>,
}

impl Wraith {
    pub fn new(config_watcher: &mut ConfigWatcher, physics_sim: &mut PhysicsSimulation) -> StatusOr<Wraith> {
        let config_manager = SimpleConfigManager::new(config_watcher, "wraith.conf")?;

        let (wraith_state, wraith_state_machine) = {
            let config = config_manager.get();
            let wraith_body = WraithBody::new(config, physics_sim.registrar(), physics_sim.get_world_mut());
            let wraith_state = WraithState::new(config.clone(), wraith_body);
            let wraith_state_machine = Box::new(WraithUpright::new());

            (wraith_state, wraith_state_machine)
        };

        Ok(Wraith {
            config_manager,
            wraith_state,
            wraith_state_machine,
        })
    }

    pub fn register(&mut self) {
        let wraith: *const Wraith = self as *const Wraith;
        self.wraith_state.register(wraith);
    }

    pub fn pre_update(&mut self, controller: &Controller, dt: DeltaTime) {
        if self.config_manager.update() || controller.just_pressed(ControlEvent::RespawnEntities) {
            self.redeploy();
        }

        if let Some(wraith_state_machine) = self.wraith_state_machine.pre_update(&mut self.wraith_state, controller, dt) {
            self.wraith_state_machine = wraith_state_machine;
        }
    }

    pub fn post_update(&mut self) {
        if let Some(wraith_state_machine) = self.wraith_state_machine.post_update() {
            self.wraith_state_machine = wraith_state_machine;
        }
    }

    fn redeploy(&mut self) {
        {
            let config = self.config_manager.get();
            let mut world = self.wraith_state.body.body.data_setter.get_world();
            let wraith_body = WraithBody::new(config, self.wraith_state.body.body.registrar.clone(), &mut world);
            self.wraith_state = WraithState::new(config.clone(), wraith_body);
            self.wraith_state_machine = Box::new(WraithUpright::new());
        }
        self.register();
    }

    pub fn take_slashing(&mut self, dir: LrDirection) {
        self.wraith_state_machine.take_slashing(&mut self.wraith_state, dir);
    }

    pub fn draw(&self, box_renderer: &mut BoxRenderer) {
        let (body_position, body_size) = (self.wraith_state.get_body_position(), self.wraith_state.config.size);
        let boxes = vec!(
            BoxData {
                position: glm::vec2(body_position.x, body_position.y),
                half_size: glm::vec2(body_size.0 as f32 / 2.0, body_size.1 as f32 / 2.0),
                rgba_tl: glm::vec4(1.0, 0.0, 0.0, 0.0),
                rgba_tr: glm::vec4(1.0, 0.0, 0.0, 0.0),
                rgba_bl: glm::vec4(1.0, 0.0, 0.2, 0.0),
                rgba_br: glm::vec4(1.0, 0.05, 0.0, 0.0),
            });

        box_renderer.queue(boxes.as_slice());
    }
}
