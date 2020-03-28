use glm;

pub struct Rotations;

impl Rotations {
    pub fn perturb_direction(inclination: f32, azimuth: f32, max_radian_offset: f32, unit_multipliers: (f32, f32)) -> glm::Vec3 {
        let rand_azimuth = (2.0 * unit_multipliers.0 - 1.0) * max_radian_offset + azimuth;
        let rand_inclination = (2.0 * unit_multipliers.1 - 1.0) * max_radian_offset + inclination;
        let sin_inc = rand_inclination.sin();
        let x_dir = sin_inc * rand_azimuth.cos();
        let y_dir = rand_inclination.cos();
        let z_dir = sin_inc * rand_azimuth.sin();
        glm::vec3(x_dir, y_dir, z_dir)
    }

    pub fn compute_inclination_and_azimuth(direction: glm::Vec3) -> (f32, f32) {
        let inclination = direction.y.acos();
        let azimuth = direction.z.atan2(direction.x);
        (inclination, azimuth)
    }
}
