use nalgebra::Point2;

pub struct BloodParticleEvent {
    pub position: Point2<f64>,
    pub color: glm::Vec3,
    pub num_particles_to_generate: u32,
}

pub struct HeroSwitchParticleEvent {
    pub position: Point2<f64>,
}

pub enum ParticleEvent {
    Blood(BloodParticleEvent),
    HeroSwitch(HeroSwitchParticleEvent),
}

impl ParticleEvent {
    pub fn blood(position: Point2<f64>, color: glm::Vec3, num_particles_to_generate: u32) -> Self {
        Self::Blood(BloodParticleEvent {
            position,
            color,
            num_particles_to_generate,
        })
    }

    pub fn hero_switch(position: Point2<f64>) -> Self {
        Self::HeroSwitch(HeroSwitchParticleEvent {
            position
        })
    }
}

