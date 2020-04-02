#version 330 core

out VS_OUT {
    vec4 color;
    vec2 texel;
} vs_out;

in vec3 left_top;
in vec2 right_bottom;
in vec2 tex_left_top;
in vec2 tex_right_bottom;
in vec4 color;

uniform mat4 projection;

const mat4 INVERT_Y_AXIS = mat4(
    vec4(1.0, 0.0, 0.0, 0.0),
    vec4(0.0, -1.0, 0.0, 0.0),
    vec4(0.0, 0.0, 1.0, 0.0),
    vec4(0.0, 0.0, 0.0, 1.0)
);

void main() {
    vs_out.color = color;

    vec2 pos = vec2(0.0);
    float left = left_top.x;
    float right = right_bottom.x;
    float top = left_top.y;
    float bottom = right_bottom.y;

    switch (gl_VertexID) {
        case 0:
            pos = vec2(left, top);
            vs_out.texel = tex_left_top;
            break;
        case 1:
            pos = vec2(right, top);
            vs_out.texel = vec2(tex_right_bottom.x, tex_left_top.y);
            break;
        case 2:
            pos = vec2(left, bottom);
            vs_out.texel = vec2(tex_left_top.x, tex_right_bottom.y);
            break;
        case 3:
            pos = vec2(right, bottom);
            vs_out.texel = tex_right_bottom;
            break;
    }

    gl_Position = INVERT_Y_AXIS * projection * vec4(pos, left_top.z, 1.0);
}