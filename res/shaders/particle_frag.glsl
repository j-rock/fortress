#version 330 core

out vec4 frag_color;

in GS_OUT {
    vec3 color;
    float alpha;
} gs_out;

void main() {
    frag_color.rgb = gs_out.color;
    frag_color.a = gs_out.alpha;
}
