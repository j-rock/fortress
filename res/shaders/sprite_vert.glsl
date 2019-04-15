#version 330 core
layout (location = 0) in vec3 world_bottom_center_position;
layout (location = 1) in vec2 half_size;
layout (location = 2) in vec4 texel_coords; // xy = bottom_left, zw = top_right.

out VS_OUT {
  vec3 world_bottom_center_position;
  vec2 half_size;
  vec2 texel_bottom_left;
  vec2 texel_top_right;
} vs_out;

void main() {
    gl_Position = vec4(world_bottom_center_position, 1.0);

    vs_out.world_bottom_center_position = world_bottom_center_position;
    vs_out.half_size = half_size;
    vs_out.texel_bottom_left = texel_coords.xy;
    vs_out.texel_top_right = texel_coords.zw;
}