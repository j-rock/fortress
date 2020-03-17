#version 330 core

out vec4 frag_color;

in GS_OUT {
    vec3 world_space_position;
    vec2 texel;
} fs_in;

struct PointLight {
    vec3 position;
    int color;
    vec3 attenuation;
};
const int MAX_NUM_LIGHTS = 339;

uniform PointLight lights[MAX_NUM_LIGHTS];
uniform int num_lights;
uniform sampler2D texture0;

vec3 UnpackLightColor(int color) {
    float red = (color >> 16) / 255.0;
    float green = ((color >> 8) & 0xFF) / 255.0;
    float blue = (color & 0xFF) / 255.0;
    return vec3(red, green, blue);
}

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

        lighting += total_attenuation * diffuse_color * UnpackLightColor(lights[i].color);
    }

    // Gamma correct
    lighting = pow(lighting, vec3(1.0/2.2));
    frag_color = vec4(lighting, 1.0);
}
