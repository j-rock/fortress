#[repr(C)]
pub struct PointLight {
    pub position: glm::Vec3,
    pub color: glm::Vec3,
    // Constant, linear, quadratic
    pub attenuation: glm::Vec3,
}
