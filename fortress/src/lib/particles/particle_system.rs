use crate::{
    app::{
        RandGen,
        StatusOr,
    },
    dimensions::time::{
        DeltaTime,
        Microseconds,
    },
    file::{
        self,
        ConfigWatcher,
        SimpleConfigManager,
    },
    particles::{
        ParticleConfig,
        ParticleEvent,
    },
    render::{
        attribute,
        Attribute,
        AttributeProgram,
        ShaderProgram,
        ShaderUniformKey,
    },
};
use gl::types::GLsizei;
use glm;
use std::ffi::CString;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum UniformKey {
    ProjectionView,
    ParticleSize,
}

impl ShaderUniformKey for UniformKey {
    fn to_cstring(self) -> CString {
        let s = match self {
            UniformKey::ProjectionView => "projection_view",
            UniformKey::ParticleSize => "particle_size",
        };
        CString::new(s).expect("Bad cstring")
    }
}

pub struct ParticleSystem {
    config: SimpleConfigManager<ParticleConfig>,
    shader_program: ShaderProgram<UniformKey>,
    attribute_program: AttributeProgram,
    attr_pos: Attribute<Vec3Attr>,
    attr_color: Attribute<Vec3Attr>,
    velocity: Vec<glm::Vec3>,
    age: Vec<Microseconds>,

    queued_events: Vec<ParticleEvent>,
}

impl ParticleSystem {
    pub fn new(config_watcher: &mut ConfigWatcher) -> StatusOr<ParticleSystem> {
        let config = SimpleConfigManager::<ParticleConfig>::from_config_resource(config_watcher, "particle.conf")?;
        let vertex = file::util::resource_path("shaders", "particle_vert.glsl");
        let fragment = file::util::resource_path("shaders", "particle_frag.glsl");
        let shader_program = ShaderProgram::from_short_pipeline(&vertex, &fragment)?;

        let mut attribute_program_builder = AttributeProgram::builder();
        let mut attr_pos = attribute_program_builder.add_attribute();
        let mut attr_color = attribute_program_builder.add_attribute();
        let attribute_program = attribute_program_builder.build();

        let (age, velocity, queued_events) = {
            let config = config.get();
            attr_pos.data.reserve(config.initial_particle_capacity_guess);
            attr_color.data.reserve(config.initial_particle_capacity_guess);
            (Vec::with_capacity(config.initial_particle_capacity_guess),
             Vec::with_capacity(config.initial_particle_capacity_guess),
             Vec::with_capacity(config.initial_particle_events_guess))
        };

        Ok(ParticleSystem {
            config,
            shader_program,
            attribute_program,
            attr_pos,
            attr_color,
            velocity,
            age,
            queued_events,
        })
    }

    pub fn respawn(&mut self) {
        self.attr_pos.data.clear();
        self.attr_color.data.clear();
        self.velocity.clear();
        self.age.clear();
        self.queued_events.clear();
    }

    pub fn pre_update(&mut self, dt: DeltaTime) {
        self.config.update();
        let config = self.config.get();
        for idx in (0..self.age.len()).rev() {
            let new_age = self.age[idx] + dt.as_microseconds();
            if new_age >= config.particle_max_age {
                self.attr_pos.data.remove(idx);
                self.attr_color.data.remove(idx);
                self.velocity.remove(idx);
                self.age.remove(idx);
                continue;
            }
            self.age[idx] = new_age;

            let position = self.attr_pos.data[idx].val;
            if position.y <= 0.0 {
                continue;
            }
            let float_dt = dt.as_f32_seconds() as f32;
            let velocity = self.velocity[idx];
            let new_velocity = glm::vec3(velocity.x, velocity.y + config.particle_gravity * float_dt, velocity.z);
            self.velocity[idx] = new_velocity;

            let mut new_pos = position + (new_velocity * float_dt);
            if new_pos.y < 0.0 {
                new_pos.y = 0.0;
            }
            self.attr_pos.data[idx] = Vec3Attr::new(new_pos);
        }
    }

    pub fn queue_event(&mut self, event: ParticleEvent) {
        self.queued_events.push(event);
    }

    pub fn post_update(&mut self, rng: &mut RandGen) {
        let config = self.config.get();
        for event in self.queued_events.iter() {
            for _idx in 0..config.particles_per_event {
                let vel_xz = rng.unit_circle_glm() * config.particle_max_spread_velocity;
                let velocity = glm::vec3(vel_xz.x, config.particle_start_velocity_y, vel_xz.y);

                let radius = rng.unit_circle_glm() * event.radius;
                let position =
                    glm::vec3(radius.x + event.position.x as f32,
                              config.particle_start_height,
                              radius.y - event.position.y as f32);

                self.attr_pos.data.push(Vec3Attr::new(position));
                self.attr_color.data.push(Vec3Attr::new(event.color * rng.unit_f32()));
                self.velocity.push(velocity);
                self.age.push(0);
            }
        }
        self.queued_events.clear();
    }

    pub fn draw(&mut self, projection_view: &glm::Mat4) {
        let config = self.config.get();

        self.shader_program.activate();
        self.attribute_program.activate();

        self.shader_program.set_mat4(UniformKey::ProjectionView, projection_view);
        self.shader_program.set_f32(UniformKey::ParticleSize, config.particle_size);

        self.attr_pos.prepare_buffer();
        self.attr_color.prepare_buffer();

        unsafe {
            gl::DrawArraysInstanced(gl::POINTS, 0, 1, self.attr_pos.data.len() as GLsizei);
        }

        self.attribute_program.deactivate();
        self.shader_program.deactivate();
    }
}

#[repr(C)]
struct Vec3Attr {
    val: glm::Vec3,
}

impl Vec3Attr {
    pub fn new(val: glm::Vec3) -> Vec3Attr {
        Vec3Attr {
            val
        }
    }
}

impl attribute::KnownComponent for Vec3Attr {
    fn component() -> (attribute::NumComponents, attribute::ComponentType) {
        (attribute::NumComponents::S3, attribute::ComponentType::Float)
    }
}
