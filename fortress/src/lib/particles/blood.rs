use crate::{
    app::RandGen,
    dimensions::time::DeltaTime,
    particles::{
        BloodParticleConfig,
        BloodParticleEvent,
        ParticleRenderView,
        particle_render_view::{
            Vec3Attr,
            FloatAttr,
        },
        RingBufferView,
    },
    render::{
        CameraStreamBounds,
        CameraStreamInfo,
        EasingFn,
    },
};
use glm;
use nalgebra::Point2;

pub struct BloodParticles {
    ring_buffer_view: RingBufferView,
    position: Vec<glm::Vec3>,
    color: Vec<glm::Vec3>,
    velocity: Vec<glm::Vec3>,
}

impl BloodParticles {
    pub fn new(config: &BloodParticleConfig) -> BloodParticles {
        let particle_limit = config.particle_limit;
        BloodParticles {
            ring_buffer_view: RingBufferView::with_capacity(particle_limit),
            position: Vec::with_capacity(particle_limit),
            color: Vec::with_capacity(particle_limit),
            velocity: Vec::with_capacity(particle_limit),
        }
    }

    pub fn respawn(&mut self) {
        self.ring_buffer_view.clear();
        self.position.clear();
        self.color.clear();
        self.velocity.clear();
    }

    pub fn pre_update(&mut self, config: &BloodParticleConfig, dt: DeltaTime) {
        for idx in 0..self.ring_buffer_view.len() {
            let position = self.position[idx];
            if position.y <= 0.0 {
                continue;
            }
            let float_dt = dt.as_f32_seconds() as f32;
            let velocity = self.velocity[idx];
            let new_velocity = glm::vec3(velocity.x, velocity.y + config.gravity * float_dt, velocity.z);
            self.velocity[idx] = new_velocity;

            let mut new_pos = position + (new_velocity * float_dt);
            if new_pos.y < 0.0 {
                new_pos.y = 0.0;
            }
            self.position[idx] = new_pos;
        }
    }

    pub fn add_event(&mut self, config: &BloodParticleConfig, event: &BloodParticleEvent, rng: &mut RandGen) {
        (0..config.particles_per_event)
            .for_each(|_idx| {
                let vel_xz = rng.unit_circle_glm() * rng.unit_f32() * config.max_spread_speed;
                let velocity = glm::vec3(vel_xz.x, config.start_velocity_y, vel_xz.y);

                let radius = rng.unit_circle_glm() * event.radius;
                let position =
                    glm::vec3(radius.x + event.position.x as f32,
                              config.start_height,
                              radius.y - event.position.y as f32);
                let color = event.color * rng.unit_f32();

                self.ring_buffer_view.add_element_at_head(position, &mut self.position);
                self.ring_buffer_view.add_element_at_head(color, &mut self.color);
                self.ring_buffer_view.add_element_at_head(velocity, &mut self.velocity);
                self.ring_buffer_view.increment_head();
            });
    }

    pub fn queue_draw(&self, config: &BloodParticleConfig, camera_stream_info: &CameraStreamInfo, render_view: ParticleRenderView) {
        (0..self.ring_buffer_view.len())
            .for_each(|idx| {
                let position = self.position[idx];
                let alpha = match camera_stream_info.compute_bounds(Point2::new(position.x as f64, -position.z as f64)) {
                    CameraStreamBounds::Outside => 0.0,
                    CameraStreamBounds::Inside => 1.0,
                    CameraStreamBounds::Margin(margin) => EasingFn::ease_in_cuartic(margin),
                };
                let color = self.color[idx];

                render_view.attr_pos.push(Vec3Attr::new(position));
                render_view.attr_color.push(Vec3Attr::new(color));
                render_view.attr_alpha.push(FloatAttr::new(alpha));
                render_view.attr_size.push(FloatAttr::new(config.size));
            });
    }
}