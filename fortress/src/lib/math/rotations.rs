use glm;

pub struct Rotations;

impl Rotations {
    pub fn perturb_direction(inclination: f32, azimuth: f32, inclination_offset: f32, azimuth_offset: f32) -> glm::Vec3 {
        let new_inclination = inclination + inclination_offset;
        let new_azimuth = azimuth + azimuth_offset;
        let sin_inc = new_inclination.sin();
        let x_dir = sin_inc * new_azimuth.cos();
        let y_dir = new_inclination.cos();
        let z_dir = sin_inc * new_azimuth.sin();
        glm::vec3(x_dir, y_dir, z_dir)
    }

    pub fn compute_inclination_and_azimuth(direction: glm::Vec3) -> (f32, f32) {
        let inclination = direction.y.acos();
        let azimuth = direction.z.atan2(direction.x);
        (inclination, azimuth)
    }
}
