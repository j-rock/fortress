use crate::{
    dimensions::time::DeltaTime,
    render::{
        CameraAngles,
        ScreenShakeConfig
    },
};

pub struct ScreenShake {
    intensity: f32,
}

impl ScreenShake {
    pub fn new() -> Self {
        ScreenShake {
            intensity: 0.0,
        }
    }

    pub fn intensify(&mut self, intensity: f32) {
        self.intensity += intensity;
    }

    pub fn pre_update(&mut self, config: &ScreenShakeConfig, dt: DeltaTime) {
        self.intensity -= config.intensity_fall_off_speed * dt.as_f32_seconds();

        if self.intensity < 0.0 {
            self.intensity = 0.0;
        }
        if self.intensity > 1.0 {
            self.intensity = 1.0;
        }
    }

    pub fn shake_rotation(&self, angles: &CameraAngles) -> CameraAngles {
        let lookat = angles.lookat();
        let right = angles.right();
        CameraAngles::new(lookat, right)
    }

    pub fn shake_position(&self, position: glm::Vec3) -> glm::Vec3 {
        position
    }
}
