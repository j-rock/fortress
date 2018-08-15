use app::StatusOr;
use control::{
    Controller,
    ControlEvent,
};
use dimensions::time::DeltaTime;
use file::{
    ConfigWatcher,
    SimpleConfigManager,
    self,
};
use gl::{
    self,
    types::*,
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
    attribute,
    Attribute,
    AttributeProgram,
    ShaderProgram,
};

#[repr(C)]
struct WraithAttr {
    position: glm::Vec2,
    half_size: glm::Vec2,
}

impl attribute::KnownComponent for WraithAttr {
    fn component() -> (attribute::NumComponents, attribute::ComponentType) {
        (attribute::NumComponents::S4, attribute::ComponentType::FLOAT)
    }
}

pub struct Wraith {
    config_manager: SimpleConfigManager<WraithConfig>,
    wraith_state: WraithState,
    wraith_state_machine: Box<dyn WraithStateMachine>,

    shader_program: ShaderProgram,
    attribute_program: AttributeProgram,
    wraith_attribute: Attribute<WraithAttr>,
}

impl Wraith {
    pub fn new(config_watcher: &mut ConfigWatcher, physics_sim: &mut PhysicsSimulation) -> StatusOr<Wraith> {
        let config_manager = SimpleConfigManager::new(config_watcher, "wraith.conf")?;

        let (wraith_state, wraith_state_machine) = {
            let config = config_manager.get();
            let wraith_body = WraithBody::new(config, physics_sim.get_world_mut());
            let wraith_state = WraithState::new(config.clone(), wraith_body);
            let wraith_state_machine = Box::new(WraithUpright::new());

            (wraith_state, wraith_state_machine)
        };

        let vertex = file::util::resource_path("shaders", "wraith_vert.glsl");
        let geometry = file::util::resource_path("shaders", "wraith_geo.glsl");
        let fragment = file::util::resource_path("shaders", "wraith_frag.glsl");
        let shader_program = ShaderProgram::from_long_pipeline(&vertex, &geometry, &fragment)?;
        let mut attribute_program_builder = AttributeProgram::new();
        let wraith_attribute = attribute_program_builder.add_attribute();
        let attribute_program = attribute_program_builder.build();

        Ok(Wraith {
            config_manager,
            wraith_state,
            wraith_state_machine,
            shader_program,
            attribute_program,
            wraith_attribute
        })
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
        let config = self.config_manager.get();
        let mut world = self.wraith_state.body.body.get_world();
        let wraith_body = WraithBody::new(config, &mut world);
        self.wraith_state = WraithState::new(config.clone(), wraith_body);
        self.wraith_state_machine = Box::new(WraithUpright::new());
    }

    pub fn draw(&mut self, projection_view: &glm::Mat4) {
        let (body_position, body_size) = (self.wraith_state.get_body_position(), self.wraith_state.config.size);
        self.wraith_attribute.data = vec!(
            WraithAttr {
                position: glm::vec2(body_position.x, body_position.y),
                half_size: glm::vec2(body_size.0 as f32 / 2.0, body_size.1 as f32 / 2.0)
            });

        self.shader_program.activate();
        self.shader_program.set_mat4("projection_view", projection_view);
        self.attribute_program.activate();
        self.wraith_attribute.prepare_buffer();
        unsafe {
            gl::DrawArraysInstanced(gl::POINTS, 0, 4, self.wraith_attribute.data.len() as GLsizei);
        }
        self.attribute_program.deactivate();
        self.shader_program.deactivate();
    }
}
