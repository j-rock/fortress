#version 330 core
layout (points) in;
layout (triangle_strip, max_vertices = 4) out;

in VS_OUT {
    vec4 screen_position;
    vec4 texel_coords;
    vec4 color;
    float screen_z;
} gs_in[];

out GS_OUT {
    vec4 color;
    vec2 texel;
} gs_out;

uniform mat4 projection;

const mat4 invert_y_axis = mat4(
    vec4(1.0, 0.0, 0.0, 0.0),
    vec4(0.0, -1.0, 0.0, 0.0),
    vec4(0.0, 0.0, 1.0, 0.0),
    vec4(0.0, 0.0, 0.0, 1.0)
);

void EmitV(vec2 screen_xy, vec2 texel) {
    gs_out.texel = texel;
    gl_Position = invert_y_axis * projection * vec4(screen_xy, gs_in[0].screen_z, 1.0);
    EmitVertex();
}

void main() {
    gs_out.color = gs_in[0].color;

    vec2 screen_bot_left = gs_in[0].screen_position.xy;
    vec2 screen_top_right = gs_in[0].screen_position.zw;
    vec2 screen_top_left = vec2(screen_bot_left.x, screen_top_right.y);
    vec2 screen_bot_right = vec2(screen_top_right.x, screen_bot_left.y);

    vec2 texel_bot_left = gs_in[0].texel_coords.xy;
    vec2 texel_top_right = gs_in[0].texel_coords.zw;
    vec2 texel_top_left = vec2(texel_bot_left.x, texel_top_right.y);
    vec2 texel_bot_right = vec2(texel_top_right.x, texel_bot_left.y);

    EmitV(screen_top_left, texel_top_left);
    EmitV(screen_top_right, texel_top_right);
    EmitV(screen_bot_left, texel_bot_left);
    EmitV(screen_bot_right, texel_bot_right);
    EndPrimitive();
}
