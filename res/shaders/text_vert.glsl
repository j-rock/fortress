#version 330 core
layout (location = 0) in vec4 screen_position; // xy = bottom_left, zw = top_right.
layout (location = 1) in vec4 texel_coords; // xy = bottom_left, zw = top_right.
layout (location = 2) in vec4 color;
layout (location = 3) in float screen_z;

out VS_OUT {
    vec4 screen_position;
    vec4 texel_coords;
    vec4 color;
    float screen_z;
} vs_out;

void main() {
    vs_out.screen_position = screen_position;
    vs_out.texel_coords = texel_coords;
    vs_out.color = color;
    vs_out.screen_z = screen_z;

    // Nonsense value.
    gl_Position = vec4(screen_position.xy, screen_z, 1.0);
}