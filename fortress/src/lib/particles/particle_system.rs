use crate::{
    app::{
        RandGen,
        StatusOr,
    },
    dimensions::time::DeltaTime,
    file::{
        self,
        ConfigWatcher,
        SimpleConfigManager,
    },
    particles::{
        BloodParticles,
        ParticleConfig,
        ParticleEvent,
        ParticleRenderView,
        particle_render_view::{
            FloatAttr,
            Vec3Attr,
        },
        SnowParticles,
    },
    render::{
        Attribute,
        AttributeProgram,
        CameraStreamInfo,
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
    CameraRight,
    CameraUp,
}

impl ShaderUniformKey for UniformKey {
    fn to_cstring(self) -> CString {
        let s = match self {
            UniformKey::ProjectionView => "projection_view",
            UniformKey::CameraRight => "camera_right",
            UniformKey::CameraUp => "camera_up",
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
    attr_alpha: Attribute<FloatAttr>,
    attr_size: Attribute<FloatAttr>,
    queued_events: Vec<ParticleEvent>,
    blood_particles: BloodParticles,
    snow_particles: SnowParticles,
}

impl ParticleSystem {
    pub fn new(config_watcher: &mut ConfigWatcher) -> StatusOr<ParticleSystem> {
        let config = SimpleConfigManager::<ParticleConfig>::from_config_resource(config_watcher, "particle.conf")?;
        let vertex = file::util::resource_path("shaders", "particle_vert.glsl");
        let geometry = file::util::resource_path("shaders", "particle_geo.glsl");
        let fragment = file::util::resource_path("shaders", "particle_frag.glsl");
        let shader_program = ShaderProgram::from_long_pipeline(&vertex, &geometry, &fragment)?;

        let mut attribute_program_builder = AttributeProgram::builder();
        let mut attr_pos = attribute_program_builder.add_attribute();
        let mut attr_color = attribute_program_builder.add_attribute();
        let mut attr_alpha = attribute_program_builder.add_attribute();
        let mut attr_size = attribute_program_builder.add_attribute();
        let attribute_program = attribute_program_builder.build();

        let (blood_particles, snow_particles, queued_events) = {
            let config = config.get();
            let total_particle_limit = config.blood.particle_limit + config.snow.particle_limit;
            attr_pos.data.reserve(total_particle_limit);
            attr_color.data.reserve(total_particle_limit);
            attr_alpha.data.reserve(total_particle_limit);
            attr_size.data.reserve(total_particle_limit);

            (BloodParticles::new(&config.blood),
             SnowParticles::new(&config.snow),
             Vec::with_capacity(config.initial_particle_events_limit_guess))
        };

        Ok(ParticleSystem {
            config,
            shader_program,
            attribute_program,
            attr_pos,
            attr_color,
            attr_alpha,
            attr_size,
            queued_events,
            blood_particles,
            snow_particles,
        })
    }

    pub fn respawn(&mut self) {
        self.attr_pos.data.clear();
        self.attr_color.data.clear();
        self.attr_alpha.data.clear();
        self.attr_size.data.clear();
        self.queued_events.clear();

        self.blood_particles.respawn();
        self.snow_particles.respawn();
    }

    pub fn pre_update(&mut self, dt: DeltaTime) {
        self.config.update();
        let config = self.config.get();
        self.blood_particles.pre_update(&config.blood, dt);
        self.snow_particles.pre_update(&config.snow, dt);
    }

    pub fn queue_event(&mut self, event: ParticleEvent) {
        self.queued_events.push(event);
    }

    pub fn post_update(&mut self, camera_stream_info: &CameraStreamInfo, rng: &mut RandGen) {
        let config = self.config.get();
        for event in self.queued_events.iter() {
            match event {
                ParticleEvent::Blood(ref event) => self.blood_particles.add_event(&config.blood, event, rng),
            }
        }
        self.queued_events.clear();

        self.snow_particles.post_update(&config.snow, camera_stream_info, rng);
    }

    pub fn draw(&mut self, camera_stream_info: &CameraStreamInfo, projection_view: &glm::Mat4, camera_right: glm::Vec3, camera_up: glm::Vec3) {
        let config = self.config.get();

        {
            let render_view = ParticleRenderView {
                attr_pos: &mut self.attr_pos.data,
                attr_color: &mut self.attr_color.data,
                attr_alpha: &mut self.attr_alpha.data,
                attr_size: &mut self.attr_size.data,
            };
            self.blood_particles.queue_draw(&config.blood, camera_stream_info, render_view);
        }
        {
            let render_view = ParticleRenderView {
                attr_pos: &mut self.attr_pos.data,
                attr_color: &mut self.attr_color.data,
                attr_alpha: &mut self.attr_alpha.data,
                attr_size: &mut self.attr_size.data,
            };
            self.snow_particles.queue_draw(&config.snow, render_view);
        }

        self.shader_program.activate();
        self.attribute_program.activate();

        self.shader_program.set_mat4(UniformKey::ProjectionView, projection_view);
        self.shader_program.set_vec3(UniformKey::CameraRight, &camera_right);
        self.shader_program.set_vec3(UniformKey::CameraUp, &camera_up);

        self.attr_pos.prepare_buffer();
        self.attr_color.prepare_buffer();
        self.attr_alpha.prepare_buffer();
        self.attr_size.prepare_buffer();

        unsafe {
            gl::DrawArraysInstanced(gl::POINTS, 0, 1, self.attr_pos.data.len() as GLsizei);
        }

        self.attr_pos.data.clear();
        self.attr_color.data.clear();
        self.attr_alpha.data.clear();
        self.attr_size.data.clear();

        self.attribute_program.deactivate();
        self.shader_program.deactivate();
    }
}
