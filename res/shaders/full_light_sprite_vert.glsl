#version 330 core
layout (location = 0) in vec3 world_center_position;
layout (location = 1) in vec2 half_size;
layout (location = 2) in vec4 texel_coords; // xy = bottom_left, zw = top_right.
layout (location = 3) in vec2 unit_world_rotation_xz;
layout (location = 4) in float bloom_intensity;

out VS_OUT {
    vec3 world_center_position;
    vec2 half_size;
    vec2 texel_bottom_left;
    vec2 texel_top_right;
    vec2 unit_world_rotation_xz;
    float bloom_intensity;
} vs_out;

void main() {
    gl_Position = vec4(world_center_position, 1.0);

    vs_out.world_center_position = world_center_position;
    vs_out.half_size = half_size;

    // Since OpenGL loads the image upside-down, invert the texel coords.
    vs_out.texel_bottom_left = vec2(texel_coords.x, 1.0 - texel_coords.y);
    vs_out.texel_top_right = vec2(texel_coords.z, 1.0 - texel_coords.w);

    vs_out.unit_world_rotation_xz = unit_world_rotation_xz;
    vs_out.bloom_intensity = bloom_intensity;
}
