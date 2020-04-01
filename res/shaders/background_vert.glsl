#version 330 core
layout (location = 0) in vec2 position;
layout (location = 1) in vec2 in_texture_coords;

out VS_OUT {
    vec2 texel;
} vs_out;

void main() {
    gl_Position = vec4(position, 0.0, 1.0);
    vs_out.texel = vec2(in_texture_coords.x, 1.0 - in_texture_coords.y);
}