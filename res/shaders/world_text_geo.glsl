#version 330 core
layout (points) in;
layout (triangle_strip, max_vertices = 4) out;

in VS_OUT {
    vec3 world_position;
    vec2 glyph_size;
    vec4 texel_coords;
    vec4 rgba_color;
} gs_in[];

out GS_OUT {
    vec2 texel;
    vec4 rgba_color;
} gs_out;

uniform vec3 camera_right;
uniform vec3 camera_up;
uniform mat4 projection_view;

void EmitV(vec3 position, vec2 texel) {
    gl_Position = projection_view * vec4(position, 1.0);
    gs_out.texel = texel;
    EmitVertex();
}

void main() {
    gs_out.rgba_color = gs_in[0].rgba_color;

    vec3 pointed_right = camera_right * gs_in[0].glyph_size.x;
    vec3 pointed_up = camera_up * gs_in[0].glyph_size.y;

    vec3 top_left = gs_in[0].world_position;
    vec3 top_right = top_left + pointed_right;
    vec3 bot_left = top_left + pointed_up;
    vec3 bot_right = bot_left + pointed_right;

    vec2 texel_bot_left = gs_in[0].texel_coords.xy;
    vec2 texel_top_right = gs_in[0].texel_coords.zw;
    vec2 texel_top_left = vec2(texel_bot_left.x, texel_top_right.y);
    vec2 texel_bot_right = vec2(texel_top_right.x, texel_bot_left.y);

    EmitV(top_left, texel_top_left);
    EmitV(top_right, texel_top_right);
    EmitV(bot_left, texel_bot_left);
    EmitV(bot_right, texel_bot_right);
    EndPrimitive();
}
