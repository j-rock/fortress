#version 330 core
layout (location = 0) in vec4 position_vec4;
layout (location = 1) in vec4 color_tl;
layout (location = 2) in vec4 color_tr;
layout (location = 3) in vec4 color_bl;
layout (location = 4) in vec4 color_br;

out VS_OUT {
  vec2 position;
  vec2 half_size;
  vec4 color_tl;
  vec4 color_tr;
  vec4 color_bl;
  vec4 color_br;
} vs_out;

void main() {
    gl_Position = position_vec4;
    vs_out.position = position_vec4.xy;
    vs_out.half_size = position_vec4.zw;
    vs_out.color_tl = color_tl;
    vs_out.color_tr = color_tr;
    vs_out.color_bl = color_bl;
    vs_out.color_br = color_br;
}
