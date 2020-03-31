#version 330 core
layout (location = 0) out vec4 frag_color;
layout (location = 1) out vec4 bloom_color;

in GS_OUT {
    vec3 world_space_position;
    vec2 texel;
} fs_in;

struct PointLight {
    vec3 position;
    vec3 color;
    vec3 attenuation;
};
const int MAX_NUM_LIGHTS = 339;

uniform PointLight lights[MAX_NUM_LIGHTS];
uniform int num_lights;
uniform sampler2D texture0;

void main() {
    vec3 position = fs_in.world_space_position;
    vec4 color = texture(texture0, fs_in.texel);
    vec3 diffuse_color = color.rgb;

    if (color.a < 0.01) {
        discard;
    }

    vec3 lighting = vec3(0.0);
    for (int i = 0; i < num_lights; i++) {
        vec3 light_displacement = lights[i].position - position;

        float distance = length(light_displacement);
        vec3 attenuation = lights[i].attenuation;
        float total_attenuation = 1.0 / (attenuation.x + (attenuation.y + attenuation.z * distance) * distance);

        lighting += total_attenuation * diffuse_color * lights[i].color;
    }

    // Gamma correct
    lighting = pow(lighting, vec3(1.0/2.2));
    frag_color = vec4(lighting, 1.0);
    bloom_color = vec4(0.0, 0.0, 0.0, 1.0);
}
