use crate::{
    app::RandGen,
    dimensions::time::DeltaTime,
    particles::{
        ParticleRenderView,
        particle_render_view::{
            Vec3Attr,
            FloatAttr,
        },
        HeroSwitchParticleEvent,
        HeroSwitchParticleConfig,
        RingBufferView,
    },
    render::{
        CameraStreamBounds,
        CameraStreamInfo,
        EasingFn,
    },
};
use glm;
use nalgebra::{
    Point2,
    Vector2,
};

pub struct HeroSwitchParticles {
    ring_buffer_view: RingBufferView,
    age: Vec<f64>,
    position_xz: Vec<Point2<f64>>,
    velocity_xz: Vec<Vector2<f64>>,
    wave_speed: Vec<f64>,
    height: Vec<f64>,
    color: Vec<glm::Vec3>,
    size: Vec<f32>,
}

impl HeroSwitchParticles {
    pub fn new(config: &HeroSwitchParticleConfig) -> Self {
        let particle_limit = config.particle_limit;
        HeroSwitchParticles {
            ring_buffer_view: RingBufferView::with_capacity(particle_limit),
            age: Vec::with_capacity(particle_limit),
            position_xz: Vec::with_capacity(particle_limit),
            velocity_xz: Vec::with_capacity(particle_limit),
            wave_speed: Vec::with_capacity(particle_limit),
            height: Vec::with_capacity(particle_limit),
            color: Vec::with_capacity(particle_limit),
            size: Vec::with_capacity(particle_limit),
        }
    }

    pub fn respawn(&mut self) {
        self.ring_buffer_view.clear();
        self.age.clear();
        self.position_xz.clear();
        self.velocity_xz.clear();
        self.wave_speed.clear();
        self.height.clear();
        self.color.clear();
        self.size.clear();
    }

    pub fn pre_update(&mut self, config: &HeroSwitchParticleConfig, dt: DeltaTime) {
        (0..self.ring_buffer_view.len())
            .rev()
            .for_each(|idx| {
                let float_dt = dt.as_f64_seconds();
                let new_age = self.age[idx] + float_dt;
                if new_age >= config.max_age_seconds {
                    self.swap_delete(idx);
                    return;
                }
                self.age[idx] = new_age;

                let position = &self.position_xz[idx];
                let velocity = &self.velocity_xz[idx];
                self.position_xz[idx] = position + velocity * float_dt;
            });
    }

    pub fn add_event(&mut self, config: &HeroSwitchParticleConfig, event: &HeroSwitchParticleEvent, rng: &mut RandGen) {
        (0..config.particles_per_event)
            .for_each(|_idx| {
                let age = 0.0;

                let (position_xz, velocity_xz) = {
                    let unit_circle_point = rng.unit_circle_point();
                    let position_xz = unit_circle_point.clone() * config.starting_radial_offset + event.position.coords.clone();
                    let speed_xz = (config.xz_speed_band.1 - config.xz_speed_band.0) * rng.unit_f64() + config.xz_speed_band.0;
                    let velocity_xz = unit_circle_point.coords * speed_xz;
                    (position_xz, velocity_xz)
                };

                let wave_speed = (config.wave_speed_band.1 - config.wave_speed_band.0) * rng.unit_f64() + config.wave_speed_band.0;
                let height = (config.starting_height_band.1 - config.starting_height_band.0) * rng.unit_f64() + config.starting_height_band.0;
                let color = glm::vec3(config.color.0, config.color.1, config.color.2) * rng.unit_f32();
                let size = config.size;

                self.ring_buffer_view.add_element_at_head(age, &mut self.age);
                self.ring_buffer_view.add_element_at_head(position_xz, &mut self.position_xz);
                self.ring_buffer_view.add_element_at_head(velocity_xz, &mut self.velocity_xz);
                self.ring_buffer_view.add_element_at_head(wave_speed, &mut self.wave_speed);
                self.ring_buffer_view.add_element_at_head(height, &mut self.height);
                self.ring_buffer_view.add_element_at_head(color, &mut self.color);
                self.ring_buffer_view.add_element_at_head(size, &mut self.size);
                self.ring_buffer_view.increment_head();
            });
    }

    pub fn queue_draw(&self, config: &HeroSwitchParticleConfig, camera_stream_info: &CameraStreamInfo, render_view: ParticleRenderView) {
        (0..self.ring_buffer_view.len())
            .for_each(|idx| {
                let position_xz = self.position_xz[idx].clone();

                let position = {
                    let position_y = config.wave_amplitude * (self.wave_speed[idx] * self.age[idx] - config.wave_phase_shift).sin() + self.height[idx];
                    glm::vec3(position_xz.x as f32, position_y as f32, -position_xz.y as f32)
                };

                let alpha = match camera_stream_info.compute_bounds(position_xz) {
                    CameraStreamBounds::Outside => 0.0,
                    CameraStreamBounds::Inside => 1.0,
                    CameraStreamBounds::Margin(margin) => EasingFn::ease_in_cuartic(margin),
                };
                let color = self.color[idx];
                let size = self.size[idx];

                render_view.attr_pos.push(Vec3Attr::new(position));
                render_view.attr_color.push(Vec3Attr::new(color));
                render_view.attr_alpha.push(FloatAttr::new(alpha));
                render_view.attr_size.push(FloatAttr::new(size));
            });
    }

    fn swap_delete(&mut self, index: usize) {
        self.ring_buffer_view.drop_last();
        self.age.swap_remove(index);
        self.position_xz.swap_remove(index);
        self.velocity_xz.swap_remove(index);
        self.wave_speed.swap_remove(index);
        self.height.swap_remove(index);
        self.color.swap_remove(index);
        self.size.swap_remove(index);
    }
}
