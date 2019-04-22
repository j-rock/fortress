use crate::render::ShaderProgram;

#[repr(C)]
pub struct PointLight {
    pub position: glm::Vec3,
    pub color: glm::Vec3,
    // Constant, linear, quadratic
    pub attenuation: glm::Vec3,
}

impl PointLight {
    pub fn set_lights(lights: &Vec<PointLight>, shader: &mut ShaderProgram) {
        let lights_len = lights.len();
        if lights_len > 100 {
            panic!("Need to update shaders to support more than {} lights", lights_len);
        }
        shader.set_i32("num_lights", lights_len as i32);

        for (idx, point_light) in lights.iter().enumerate() {
            let position_str = format!("lights[{}].position", idx);
            let color_str = format!("lights[{}].color", idx);
            let attenuation_str = format!("lights[{}].attenuation", idx);
            shader.set_vec3(position_str.as_str(), &point_light.position);
            shader.set_vec3(color_str.as_str(), &point_light.color);
            shader.set_vec3(attenuation_str.as_str(), &point_light.attenuation);
        }
    }
}
