#version 330 core
layout (location = 0) out vec4 frag_color;
layout (location = 1) out vec4 bloom_color;

in GS_OUT {
    vec3 color;
    float alpha;
} gs_out;

void main() {
    frag_color.rgb = gs_out.color;
    frag_color.a = gs_out.alpha;
    bloom_color = vec4(0.0, 0.0, 0.0, 1.0);
}
