use glm;

pub struct CameraGeometry {
    // Isometric angles don't factor in game camera's actual position or lookat direction.
    pub isometric_right: glm::Vec3,
    pub isometric_up: glm::Vec3,
    pub isometric_view: glm::Mat4,

    pub world_position: glm::Vec3,
    pub projection_view: glm::Mat4,
}

pub struct CameraAngles {
    lookat: glm::Vec3,
    right: glm::Vec3,
    up: glm::Vec3,
}

impl CameraAngles {
    pub fn new(lookat: glm::Vec3, right: glm::Vec3) -> Self {
        let lookat = glm::builtin::normalize(lookat);
        let right = glm::builtin::normalize(right);
        CameraAngles {
            lookat,
            right,
            up: glm::builtin::cross(right, lookat),
        }
    }

    pub fn lookat(&self) -> glm::Vec3 {
        self.lookat
    }

    pub fn right(&self) -> glm::Vec3 {
        self.right
    }

    pub fn up(&self) -> glm::Vec3 {
        self.up
    }
}

