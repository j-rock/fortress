#version 330 core
layout (location = 0) out vec4 frag_color;
layout (location = 1) out vec4 bloom_color;

in GS_OUT {
    vec3 world_space_position;
    vec3 normal;
    float alpha;
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
uniform vec2 tile_bottom_left;
uniform vec2 tile_top_right;
uniform vec2 tile_scale;

vec2 ComputeFractional(vec2 in_tex) {
    vec2 gap = tile_top_right - tile_bottom_left;
    vec2 cleaned = in_tex * tile_scale;
    return fract(cleaned / gap);
}

vec2 ComputeTexelCoords(vec2 in_tex) {
    vec2 fractional = ComputeFractional(in_tex);
    float epsilon = 0.001;
    return vec2(mix(tile_bottom_left.x, tile_top_right.x, fractional.x),
                mix(1.0 - (tile_top_right.y - epsilon), 1.0 - (tile_bottom_left.y + epsilon), fractional.y));
}

vec4 ComputeColor(vec2 in_tex) {
    vec2 fractional = ComputeFractional(in_tex);
    return vec4(fractional.x, fractional.y, 0.4, 1.0);
}

void main() {
    vec3 position = fs_in.world_space_position;
    vec3 normal = fs_in.normal;
    vec2 texel_coords = vec2(position.x, position.z - position.y);
    // vec4 color = ComputeColor(texel_coords);
    vec4 color = texture(texture0, ComputeTexelCoords(texel_coords));
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

        float diffuse_intensity = max(dot(normal, normalize(light_displacement)), 0.0);

        lighting += total_attenuation * diffuse_intensity * diffuse_color * lights[i].color;
    }

    // Gamma correct
    lighting = pow(lighting, vec3(1.0/2.2));
    frag_color = vec4(lighting, fs_in.alpha);
    bloom_color = vec4(0.0, 0.0, 0.0, 1.0);
}