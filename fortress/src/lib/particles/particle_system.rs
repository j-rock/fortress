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
        ParticleConfig,
        ParticleEvent,
        RingBufferView,
    },
    render::{
        attribute,
        Attribute,
        AttributeProgram,
        CameraStreamBounds,
        CameraStreamInfo,
        EasingFn,
        ShaderProgram,
        ShaderUniformKey,
    },
};
use gl::types::GLsizei;
use glm;
use nalgebra::Point2;
use std::ffi::CString;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum UniformKey {
    ProjectionView,
    ParticleSize,
    CameraRight,
    CameraUp,
}

impl ShaderUniformKey for UniformKey {
    fn to_cstring(self) -> CString {
        let s = match self {
            UniformKey::ProjectionView => "projection_view",
            UniformKey::ParticleSize => "particle_size",
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
    attr_alpha: Attribute<AlphaAttr>,
    velocity: Vec<glm::Vec3>,
    ring_buffer_view: RingBufferView,
    queued_events: Vec<ParticleEvent>,
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
        let attribute_program = attribute_program_builder.build();

        let (velocity, queued_events, ring_buffer_view) = {
            let config = config.get();
            attr_pos.data.reserve(config.particle_capacity);
            attr_color.data.reserve(config.particle_capacity);
            attr_alpha.data.reserve(config.particle_capacity);
            (Vec::with_capacity(config.particle_capacity),
             Vec::with_capacity(config.initial_particle_events_guess),
             RingBufferView::with_capacity(config.particle_capacity))
        };

        Ok(ParticleSystem {
            config,
            shader_program,
            attribute_program,
            attr_pos,
            attr_color,
            attr_alpha,
            velocity,
            ring_buffer_view,
            queued_events,
        })
    }

    pub fn respawn(&mut self) {
        self.attr_pos.data.clear();
        self.attr_color.data.clear();
        self.attr_alpha.data.clear();
        self.velocity.clear();
        self.ring_buffer_view.clear();
        self.queued_events.clear();
    }

    pub fn pre_update(&mut self, dt: DeltaTime) {
        self.config.update();
        let config = self.config.get();
        for idx in 0..self.ring_buffer_view.len() {
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
                let vel_xz = rng.unit_circle_glm() * rng.unit_f32() * config.particle_max_spread_velocity;
                let velocity = glm::vec3(vel_xz.x, config.particle_start_velocity_y, vel_xz.y);

                let radius = rng.unit_circle_glm() * event.radius;
                let position =
                    glm::vec3(radius.x + event.position.x as f32,
                              config.particle_start_height,
                              radius.y - event.position.y as f32);

                self.ring_buffer_view.add_element_at_head(Vec3Attr::new(position), &mut self.attr_pos.data);
                self.ring_buffer_view.add_element_at_head(Vec3Attr::new(event.color * rng.unit_f32()), &mut self.attr_color.data);
                self.ring_buffer_view.add_element_at_head(AlphaAttr::new(1.0), &mut self.attr_alpha.data);
                self.ring_buffer_view.add_element_at_head(velocity, &mut self.velocity);
                self.ring_buffer_view.increment_head();
            }
        }
        self.queued_events.clear();
    }

    pub fn draw(&mut self, camera_stream_info: &CameraStreamInfo, projection_view: &glm::Mat4, camera_right: glm::Vec3, camera_up: glm::Vec3) {
        let config = self.config.get();

        for idx in 0..self.ring_buffer_view.len() {
            let position = self.attr_pos.data[idx].val;
            let alpha = match camera_stream_info.compute_bounds(Point2::new(position.x as f64, -position.z as f64)) {
                CameraStreamBounds::Outside => 0.0,
                CameraStreamBounds::Inside => 1.0,
                CameraStreamBounds::Margin(margin) => EasingFn::ease_in_cuartic(margin),
            };
            self.attr_alpha.data[idx].set_alpha(alpha);
        }

        self.shader_program.activate();
        self.attribute_program.activate();

        self.shader_program.set_mat4(UniformKey::ProjectionView, projection_view);
        self.shader_program.set_f32(UniformKey::ParticleSize, config.particle_size);
        self.shader_program.set_vec3(UniformKey::CameraRight, &camera_right);
        self.shader_program.set_vec3(UniformKey::CameraUp, &camera_up);

        self.attr_pos.prepare_buffer();
        self.attr_color.prepare_buffer();
        self.attr_alpha.prepare_buffer();

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

#[repr(C)]
struct AlphaAttr {
    alpha: f32,
}

impl AlphaAttr {
    pub fn new(alpha: f32) -> Self {
        AlphaAttr {
            alpha
        }
    }

    pub fn set_alpha(&mut self, alpha: f32) {
        self.alpha = alpha;
    }
}

impl attribute::KnownComponent for AlphaAttr {
    fn component() -> (attribute::NumComponents, attribute::ComponentType) {
        (attribute::NumComponents::S1, attribute::ComponentType::Float)
    }
}
