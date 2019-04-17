#version 330 core

out vec4 frag_color;

in GS_OUT {
    vec3 world_space_position;
    vec4 rgba_color;
    vec3 normal;
} fs_in;

struct PointLight {
    vec3 position;
    vec3 color;
    vec3 attenuation;
};
const int MAX_NUM_LIGHTS = 100;

uniform PointLight lights[MAX_NUM_LIGHTS];
uniform int num_lights;

void main() {
    vec3 position = fs_in.world_space_position;
    vec3 normal = fs_in.normal;
    vec3 diffuse_color = fs_in.rgba_color.rgb;

    if (fs_in.rgba_color.a < 0.01) {
        discard;
    }

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