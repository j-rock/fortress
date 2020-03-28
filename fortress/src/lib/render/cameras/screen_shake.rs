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
        if self.intensity > 1.0 {
            self.intensity = 1.0;
        }
    }

    pub fn shake_rotation(&self, config: &ScreenShakeConfig, angles: &CameraAngles) -> CameraAngles {
        if self.intensity == 0.0 {
            return angles.clone();
        }

        let noise_seed1 = self.acceptably_inaccurate_time_since_construction * config.noise_time_multiplier;
        let noise_seed2 = self.acceptably_inaccurate_time_since_construction * config.noise_time_multiplier + config.noise_seed_offset;
        let shake_force = EasingFn::ease_in_quad(self.intensity);
        let unit_multipliers = (
            shake_force * NoiseFn::custom(noise_seed1, self.intensity, config.noise_iterations),
            shake_force * NoiseFn::custom(noise_seed2, self.intensity, config.noise_iterations));

        let (inclination, azimuth) = Rotations::compute_inclination_and_azimuth(angles.lookat());
        let lookat = Rotations::perturb_direction(inclination, azimuth, config.max_rotation_radians, unit_multipliers);
        let rightish = glm::cross(lookat, glm::vec3(0.0, 1.0, 0.0));
        CameraAngles::new(lookat, rightish)
    }

    pub fn shake_position(&self, position: glm::Vec3) -> glm::Vec3 {
        position
    }
}
