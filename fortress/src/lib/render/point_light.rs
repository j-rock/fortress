use crate::{
    app::StatusOr,
    file::{
        self,
        Config,
    },
    render::CameraStreamInfo,
};
use nalgebra::Point2;

#[repr(C)]
pub struct PointLight {
    pub position: glm::Vec3,
    pub color: glm::Vec3,
    // Constant, linear, quadratic
    pub attenuation: glm::Vec3,
}

#[derive(Deserialize)]
struct PointLightsConfig {
   initial_capacity: usize
}

pub struct PointLights {
    max_num_lights: usize,
    lights: Vec<PointLight>,
    camera_stream_info: Option<CameraStreamInfo>,
}

impl PointLights {
    pub fn new() -> StatusOr<Self> {
        let config_path = file::util::resource_path("config", "lights.conf");
        let config = PointLightsConfig::from_path(&config_path)?;

        Ok(PointLights {
            max_num_lights: config.initial_capacity,
            lights: Vec::with_capacity(config.initial_capacity),
            camera_stream_info: None,
        })
    }

    pub fn len(&self) -> usize {
        self.lights.len()
    }

    pub fn iter(&self) -> impl Iterator<Item=&PointLight> {
        self.lights.iter()
    }

    pub fn clear(&mut self) {
        self.lights.clear();
    }

    pub fn set_camera_stream_info(&mut self, camera_stream_info: CameraStreamInfo) {
        self.camera_stream_info = Some(camera_stream_info);
    }

    pub fn append(&mut self, lights: impl Iterator<Item=PointLight>) {
        let mut lights: Vec<PointLight> =
            lights
                .filter(|point_light| {
                    if let Some(ref camera_stream_info) = self.camera_stream_info {
                        camera_stream_info.is_point_within_light_margin(Point2::new(point_light.position.x as f64, -point_light.position.z as f64))
                    } else {
                        true
                    }
                })
                .collect();
        self.lights.append(&mut lights);

        if self.lights.len() > self.max_num_lights {
            panic!("Need to update shaders to support more than {} lights", self.lights.len());
        }
    }
}
