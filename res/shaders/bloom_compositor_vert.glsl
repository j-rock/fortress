#version 330 core
layout (location = 0) in vec3 position;
layout (location = 1) in vec2 texel_coords;

out VS_OUT {
    vec2 texel;
} vs_out;

void main() {
    gl_Position = vec4(position, 1.0);
    vs_out.texel = texel_coords;
}
