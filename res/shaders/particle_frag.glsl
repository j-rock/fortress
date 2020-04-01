#version 330 core
layout (location = 0) out vec4 frag_color;
layout (location = 1) out vec4 bloom_color;

in GS_OUT {
    vec3 color;
    vec3 bloom_color;
    float alpha;
} gs_out;

void main() {
    frag_color.rgb = gs_out.color;
    frag_color.a = gs_out.alpha;
    bloom_color = vec4(gs_out.bloom_color, 1.0);
}
