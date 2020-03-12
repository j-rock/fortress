use nalgebra::Point2;

pub struct BloodParticleEvent {
    pub position: Point2<f64>,
}

pub struct HeroSwitchParticleEvent {
    pub position: Point2<f64>,
}

pub enum ParticleEvent {
    Blood(BloodParticleEvent),
    HeroSwitch(HeroSwitchParticleEvent),
}

impl ParticleEvent {
    pub fn blood(position: Point2<f64>) -> Self {
        Self::Blood(BloodParticleEvent {
            position,
        })
    }

    pub fn hero_switch(position: Point2<f64>) -> Self {
        Self::HeroSwitch(HeroSwitchParticleEvent {
            position
        })
    }
}

