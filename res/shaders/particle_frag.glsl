#version 330 core
layout (location = 0) out vec4 frag_color;
layout (location = 1) out vec4 bloom_color;

in GS_OUT {
    vec3 color;
    vec3 bloom_color;
    float bloom_intensity;
    float alpha;
} fs_in;

void main() {
    frag_color = vec4(fs_in.color, fs_in.alpha);
    bloom_color = vec4(fs_in.bloom_color, fs_in.bloom_intensity * fs_in.alpha);
}
