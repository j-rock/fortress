use crate::{
    data::RingBufferView,
    dimensions::time::{
        DeltaTime,
        Timer,
    },
    math::{
        EasingFn,
        RandGen,
    },
    particles::{
        BloodParticleConfig,
        BloodParticleEvent,
        ParticleRenderView,
        particle_render_view::{
            BloomAttr,
            Vec3Attr,
            FloatAttr,
        },
    },
};
use glm;

pub struct BloodParticles {
    ring_buffer_view: RingBufferView,
    timer: Vec<Timer>,
    position: Vec<glm::Vec3>,
    color: Vec<glm::Vec3>,
    velocity: Vec<glm::Vec3>,
    size: Vec<f32>,
}

impl BloodParticles {
    pub fn new(config: &BloodParticleConfig) -> BloodParticles {
        let particle_limit = config.particle_limit;
        BloodParticles {
            ring_buffer_view: RingBufferView::with_capacity(particle_limit),
            timer: Vec::with_capacity(particle_limit),
            position: Vec::with_capacity(particle_limit),
            color: Vec::with_capacity(particle_limit),
            velocity: Vec::with_capacity(particle_limit),
            size: Vec::with_capacity(particle_limit),
        }
    }

    pub fn respawn(&mut self) {
        self.ring_buffer_view.clear();
        self.timer.clear();
        self.position.clear();
        self.color.clear();
        self.velocity.clear();
        self.size.clear();
    }

    pub fn pre_update(&mut self, config: &BloodParticleConfig, dt: DeltaTime) {
        (0..self.ring_buffer_view.len())
            .rev()
            .for_each(|idx| {
                let ref mut timer = self.timer[idx];
                timer.tick(dt);
                if timer.is_expired() {
                    self.swap_delete(idx);
                    return;
                }

                let position = self.position[idx];
                if position.y <= config.min_height {
                    return;
                }

                let float_dt = dt.as_f32_seconds() as f32;
                let velocity = self.velocity[idx];
                let new_velocity = glm::vec3(velocity.x, velocity.y + config.gravity * float_dt, velocity.z);
                self.velocity[idx] = new_velocity;

                let mut new_pos = position + (new_velocity * float_dt);
                if new_pos.y < config.min_height {
                    new_pos.y = config.min_height;
                }
                self.position[idx] = new_pos;
            });
    }

    pub fn add_event(&mut self, config: &BloodParticleConfig, event: &BloodParticleEvent, rng: &mut RandGen) {
        (0..event.num_particles_to_generate)
            .for_each(|_idx| {
                let vel_xz = rng.unit_circle_glm() * rng.ranged_f32(0.0, config.max_spread_speed);
                let velocity = glm::vec3(vel_xz.x, config.start_velocity_y, vel_xz.y);

                let radius = rng.unit_circle_glm() * config.start_position_radius;
                let position =
                    glm::vec3(radius.x + event.position.x as f32,
                              config.start_height,
                              radius.y - event.position.y as f32);
                let color = event.color * rng.unit_f32();
                let size = rng.ranged_f32(config.size_range.0, config.size_range.1);

                let time_left = Timer::new(config.expiry_duration_micros);

                self.ring_buffer_view.add_element_at_head(time_left, &mut self.timer);
                self.ring_buffer_view.add_element_at_head(position, &mut self.position);
                self.ring_buffer_view.add_element_at_head(color, &mut self.color);
                self.ring_buffer_view.add_element_at_head(velocity, &mut self.velocity);
                self.ring_buffer_view.add_element_at_head(size, &mut self.size);
                self.ring_buffer_view.increment_head();
            });
    }

    pub fn queue_draw(&self, config: &BloodParticleConfig, render_view: ParticleRenderView) {
        (0..self.ring_buffer_view.len())
            .for_each(|idx| {
                let position = self.position[idx];
                let color = self.color[idx];

                let t = self.timer[idx].as_completion_fraction_of(config.expiry_duration_micros);
                let drop_off = EasingFn::ease_out_quintic(1.0 - t);
                let size = self.size[idx] * (0.5 * drop_off + 0.5);
                let alpha = drop_off;

                render_view.attr_pos.push(Vec3Attr::new(position));
                render_view.attr_color.push(Vec3Attr::new(color));
                render_view.attr_bloom.push(BloomAttr::new(color, config.bloom_intensity));
                render_view.attr_alpha.push(FloatAttr::new(alpha));
                render_view.attr_size.push(FloatAttr::new(size));
            });
    }

    fn swap_delete(&mut self, index: usize) {
        self.ring_buffer_view.drop_last();
        self.timer.swap_remove(index);
        self.position.swap_remove(index);
        self.color.swap_remove(index);
        self.velocity.swap_remove(index);
        self.size.swap_remove(index);
    }
}