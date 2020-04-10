#version 330 core
layout (points) in;
layout (triangle_strip, max_vertices = 4) out;

in VS_OUT {
    vec3 screen_position;
    vec2 glyph_size;
    vec4 texel_coords;
    vec4 rgba_color;
} gs_in[];

out GS_OUT {
    vec2 texel;
    vec4 rgba_color;
} gs_out;

uniform vec2 screen_window_size;

void EmitV(mat4 projection, vec3 position, vec2 texel) {
    gl_Position = projection * vec4(position, 1.0);
    gs_out.texel = texel;
    EmitVertex();
}

void main() {
    gs_out.rgba_color = gs_in[0].rgba_color;

    vec3 top_left = gs_in[0].screen_position;
    vec3 top_right = top_left + vec3(gs_in[0].glyph_size.x, 0.0, 0.0);
    vec3 bot_left = top_left + vec3(0.0, gs_in[0].glyph_size.y, 0.0);
    vec3 bot_right = bot_left + vec3(gs_in[0].glyph_size.x, 0.0, 0.0);

    vec2 texel_bot_left = gs_in[0].texel_coords.xy;
    vec2 texel_top_right = gs_in[0].texel_coords.zw;
    vec2 texel_top_left = vec2(texel_bot_left.x, texel_top_right.y);
    vec2 texel_bot_right = vec2(texel_top_right.x, texel_bot_left.y);

    float near = 0.0;
    float far = -2.0;
    // Orthographic screen projection.
    mat4 ortho_projection = mat4(
        vec4(2.0 / screen_window_size.x, 0.0, 0.0, 0.0),
        vec4(0.0, 2.0 / screen_window_size.y, 0.0, 0.0),
        vec4(0.0, 0.0, -2.0 / (far - near), 0.0),
        vec4(-1.0, -1.0, -(far + near) / (far - near), 1.0)
    );

    EmitV(ortho_projection, top_left, texel_top_left);
    EmitV(ortho_projection, top_right, texel_top_right);
    EmitV(ortho_projection, bot_left, texel_bot_left);
    EmitV(ortho_projection, bot_right, texel_bot_right);
    EndPrimitive();
}
