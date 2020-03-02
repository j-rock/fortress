use glm;
use nalgebra::Point2;

pub struct BloodParticleEvent {
    pub position: Point2<f64>,
    pub color: glm::Vec3,
    pub radius: f32,
}

pub enum ParticleEvent {
    Blood(BloodParticleEvent),
}

impl ParticleEvent {
    pub fn blood(position: Point2<f64>, color: glm::Vec3, radius: f32) -> Self {
        Self::Blood(BloodParticleEvent {
            position,
            color,
            radius
        })
    }
}

