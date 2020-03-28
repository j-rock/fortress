use crate::{
    dimensions::time::DeltaTime,
    math::{
        EasingFn,
        NoiseFn,
        Rotations
    },
    render::{
        CameraAngles,
        ScreenShakeConfig
    },
};

pub struct ScreenShake {
    acceptably_inaccurate_time_since_construction: f32,
    intensity: f32,
}

impl ScreenShake {
    pub fn new() -> Self {
        ScreenShake {
            acceptably_inaccurate_time_since_construction: 0.0,
            intensity: 0.0,
        }
    }

    pub fn intensify(&mut self, intensity: f32) {
        self.intensity += intensity;
    }

    pub fn pre_update(&mut self, config: &ScreenShakeConfig, dt: DeltaTime) {
        let dt_f32 = dt.as_f32_seconds();
        self.acceptably_inaccurate_time_since_construction += dt_f32;

        self.intensity -= config.intensity_fall_off_speed * dt_f32;
        if self.intensity < 0.0 {
            self.intensity = 0.0;
        }
    }

    pub fn shake_rotation(&self, config: &ScreenShakeConfig, angles: &CameraAngles) -> CameraAngles {
        let (inclination_offset, azimuth_offset) = {
            let intensity = if self.intensity > 1.0 { 1.0 } else { self.intensity };

            let noise_seed1 = self.acceptably_inaccurate_time_since_construction * config.noise_time_multiplier;
            let noise_seed2 = self.acceptably_inaccurate_time_since_construction * config.noise_time_multiplier + config.noise_seed_offset;

            let noise1 = 2.0 * NoiseFn::custom(noise_seed1, intensity, config.noise_iterations) - 1.0;
            let noise2 = 2.0 * NoiseFn::custom(noise_seed2, intensity, config.noise_iterations) - 1.0;

            let shake_force = EasingFn::ease_in_cubic(intensity);
            let inclination_offset = shake_force * config.max_rotation_radians * noise1;
            let azimuth_offset = shake_force * config.max_rotation_radians * noise2;
            (inclination_offset, azimuth_offset)
        };

        let (inclination, azimuth) = Rotations::compute_inclination_and_azimuth(angles.lookat());
        let lookat = Rotations::perturb_direction(inclination, azimuth, inclination_offset, azimuth_offset);
        let right = glm::vec3(1.0, 0.0, 0.0);

        CameraAngles::new(lookat, right)
    }

    pub fn shake_position(&self, position: glm::Vec3) -> glm::Vec3 {
        position
    }
}
