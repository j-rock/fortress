use glm;

pub struct UnitQuaternion {
    w: f32,
    xyz: glm::Vec3,
}

impl UnitQuaternion {
    pub fn from_unit_axis_and_angle(axis: glm::Vec3, radians: f32) -> Self {
        let half_angle = radians / 2.0;
        let w = half_angle.cos();
        let xyz = axis * half_angle.sin();
        UnitQuaternion {
            w,
            xyz,
        }
    }

    pub fn then_rotate_by(self, that: UnitQuaternion) -> Self {
        let new_w = self.w * that.w - glm::dot(self.xyz, that.xyz);
        let new_xyz = (self.xyz * that.w) + (that.xyz * self.w) + glm::cross(self.xyz, that.xyz);

        UnitQuaternion {
            w: new_w,
            xyz: new_xyz
        }
    }

    pub fn rotate(&self, vector: glm::Vec3) -> glm::Vec3 {
        let x = vector * self.w;
        let crossed = glm::cross(self.xyz, vector);

        let term1 = self.xyz * glm::dot(self.xyz, vector);
        let term2 = x * self.w;
        let term3 = crossed * self.w;
        let term4 = glm::cross(self.xyz, x + crossed);

        term1 + term2 + term3 + term4
    }
}