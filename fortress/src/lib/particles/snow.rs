use crate::{
    app::RandGen,
    dimensions::time::{
        DeltaTime,
        Microseconds,
    },
    particles::{
        ParticleRenderView,
        particle_render_view::{
            Vec3Attr,
            FloatAttr,
        },
        RingBufferView,
        SnowParticleConfig,
    },
    render::{
        CameraStreamInfo,
        EasingFn,
    },
};
use glm;

pub struct SnowParticles {
    ring_buffer_view: RingBufferView,
    position: Vec<glm::Vec3>,
    velocity: Vec<glm::Vec3>,
    color: Vec<glm::Vec3>,
    size: Vec<f32>,
    time_since_last_snowflake: Microseconds,
    wind_inclination: f32,
    wind_azimuth: f32,
}

impl SnowParticles {
    pub fn new(config: &SnowParticleConfig) -> SnowParticles {
        let wind_direction = glm::normalize(glm::vec3(config.wind_direction_raw.0, config.wind_direction_raw.1, config.wind_direction_raw.2));
        let (wind_inclination, wind_azimuth) = Self::compute_inclination_and_azimuth(wind_direction);

        let particle_limit = config.particle_limit;
        SnowParticles {
            ring_buffer_view: RingBufferView::with_capacity(particle_limit),
            position: Vec::with_capacity(particle_limit),
            velocity: Vec::with_capacity(particle_limit),
            color: Vec::with_capacity(particle_limit),
            size: Vec::with_capacity(particle_limit),
            time_since_last_snowflake: 0,
            wind_inclination,
            wind_azimuth,
        }
    }

    pub fn respawn(&mut self) {
        self.ring_buffer_view.clear();
        self.position.clear();
        self.velocity.clear();
        self.color.clear();
        self.size.clear();
    }

    pub fn pre_update(&mut self, config: &SnowParticleConfig, dt: DeltaTime) {
        {
            let wind_direction = glm::normalize(glm::vec3(config.wind_direction_raw.0, config.wind_direction_raw.1, config.wind_direction_raw.2));
            let (wind_inclination, wind_azimuth) = Self::compute_inclination_and_azimuth(wind_direction);
            self.wind_inclination = wind_inclination;
            self.wind_azimuth = wind_azimuth;
        }
        self.time_since_last_snowflake += dt.as_microseconds();

        (0..self.ring_buffer_view.len())
            .rev()
            .for_each(|idx| {
                let old_position = self.position[idx];
                let velocity = self.velocity[idx];

                let new_position = old_position + (velocity * dt.as_f32_seconds());
                if new_position.y <= 0.0 {
                    self.swap_delete(idx);
                    return;
                }

                self.position[idx] = new_position;
            });
    }

    pub fn post_update(&mut self, config: &SnowParticleConfig, camera_stream_info: &CameraStreamInfo, rng: &mut RandGen) {
        if self.time_since_last_snowflake < config.particle_generation_period_micros {
            return;
        }
        self.time_since_last_snowflake = 0;

        let position = {
            let position = camera_stream_info.random_point_inside_bounds(rng);
            glm::vec3(position.x as f32 + config.start_position_offset.0, config.start_position_offset.1, -position.y as f32 + config.start_position_offset.2)
        };

        let velocity = {
            let rand_azimuth = (2.0 * rng.unit_f32() - 1.0) * config.wind_direction_max_angle_offset + self.wind_azimuth;
            let rand_inclination = (2.0 * rng.unit_f32() - 1.0) * config.wind_direction_max_angle_offset + self.wind_inclination;
            let sin_inc = rand_inclination.sin();
            let x_dir = sin_inc * rand_azimuth.cos();
            let y_dir = rand_inclination.cos();
            let z_dir = sin_inc * rand_azimuth.sin();
            let direction = glm::vec3(x_dir, y_dir, z_dir);
            let speed = config.speed_range.0 + (config.speed_range.1 - config.speed_range.0) * rng.unit_f32();
            direction * speed
        };

        let color = glm::vec3(config.color.0, config.color.1, config.color.2) * rng.unit_f32();
        let size = config.size_range.0 + (config.size_range.1 - config.size_range.0) * rng.unit_f32();

        self.ring_buffer_view.add_element_at_head(position, &mut self.position);
        self.ring_buffer_view.add_element_at_head(velocity, &mut self.velocity);
        self.ring_buffer_view.add_element_at_head(color, &mut self.color);
        self.ring_buffer_view.add_element_at_head(size, &mut self.size);
        self.ring_buffer_view.increment_head();
    }

    pub fn queue_draw(&self, config: &SnowParticleConfig, render_view: ParticleRenderView) {
        (0..self.ring_buffer_view.len())
            .for_each(|idx| {
                let position = self.position[idx];
                let alpha = if position.y >= config.height_above_which_alpha_is_full {
                    1.0
                } else {
                    let margin = position.y / config.height_above_which_alpha_is_full;
                    EasingFn::ease_in_cuartic(margin)
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
        self.position.swap_remove(index);
        self.velocity.swap_remove(index);
        self.color.swap_remove(index);
        self.size.swap_remove(index);
    }

    fn compute_inclination_and_azimuth(wind_direction: glm::Vec3) -> (f32, f32) {
        let inclination = wind_direction.y.acos();
        let azimuth = wind_direction.z.atan2(wind_direction.x);
        (inclination, azimuth)
    }
}
