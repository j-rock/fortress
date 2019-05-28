use glm;
use nalgebra;

pub struct ParticleEvent {
    pub position: nalgebra::Point2<f64>,
    pub color: glm::Vec3,
    pub radius: f32,
}