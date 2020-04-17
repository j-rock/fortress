#version 330 core
layout (location = 0) in vec3 position;
layout (location = 1) in vec3 color;
layout (location = 2) in vec4 bloom;
layout (location = 3) in float alpha;
layout (location = 4) in float size;

out VS_OUT {
    vec3 position;
    vec3 color;
    vec3 bloom_color;
    float bloom_intensity;
    float alpha;
    float size;
} vs_out;

void main() {
    gl_Position = vec4(position, 1.0);
    vs_out.position = position;
    vs_out.color = color;
    vs_out.bloom_color = bloom.rgb;
    vs_out.bloom_intensity = bloom.a;
    vs_out.alpha = alpha;
    vs_out.size = size;
}
