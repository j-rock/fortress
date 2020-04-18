#version 330 core
layout (location = 0) in vec3 world_position;
layout (location = 1) in vec2 glyph_size;
layout (location = 2) in vec4 texel_coords; // xy = bottom_left, zw = top_right
layout (location = 3) in vec4 rgba_color;

out VS_OUT {
    vec3 world_position;
    vec2 glyph_size;
    vec4 texel_coords;
    vec4 rgba_color;
} vs_out;

void main() {
    gl_Position = vec4(world_position, 1.0);
    vs_out.world_position = world_position;
    vs_out.glyph_size = glyph_size;
    vs_out.texel_coords = texel_coords;
    vs_out.rgba_color = rgba_color;
}
