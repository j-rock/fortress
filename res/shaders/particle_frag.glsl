#version 330 core

out vec4 frag_color;

in VS_OUT {
    vec3 color;
} vs_out;

void main() {
    frag_color = vec4(vs_out.color, 1.0);
}
