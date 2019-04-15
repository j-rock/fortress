#version 330 core

out vec4 frag_color;
  
in vec2 texture_coords;

struct PointLight {
	vec3 position;
	vec3 color;
    vec3 attenuation;
};
const int MAX_NUM_LIGHTS = 32;

uniform sampler2D position_tex;
uniform sampler2D normal_tex;
uniform sampler2D color_tex;
uniform PointLight lights[MAX_NUM_LIGHTS];
uniform int num_lights;

void main() {
    vec3 position = texture(position_tex, texture_coords).rgb;
    vec3 normal = texture(normal_tex, texture_coords).rgb;
	vec4 diffuse_specular = texture(color_tex, texture_coords);
	vec3 diffuse_color = diffuse_specular.rgb;
	float specular_color = diffuse_specular.a;

	vec3 lighting = vec3(0.0);

    for (int i = 0; i < num_lights; i++) {
		vec3 light_displacement = lights[i].position - position;

		float distance = length(light_displacement);
        vec3 attenuation = lights[i].attenuation;
		float total_attenuation = 1.0 / (attenuation.x + (attenuation.y + attenuation.z * distance) * distance);

		float diffuse_intensity = max(dot(normal, normalize(light_displacement)), 0.0);

		lighting += total_attenuation * diffuse_intensity * diffuse_color * lights[i].color;
	}

	// Gamma correct
	lighting = pow(lighting, vec3(1.0/2.2));
    frag_color = vec4(lighting, 1.0);
}