#version 330 core
layout (location = 0) out vec3 position;
layout (location = 1) out vec3 normal;
layout (location = 2) out vec4 color; // rgb = diffuse, a = specular

in GS_OUT {
    vec3 world_space_position;
    vec4 rgba_color;
    vec3 normal;
} fs_in;

void main() {
    position = fs_in.world_space_position;
    normal = fs_in.normal;
    color.rgb = fs_in.rgba_color.rgb;
    color.a = fs_in.rgba_color.a;
}