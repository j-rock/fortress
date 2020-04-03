#version 330 core
layout (location = 0) out vec4 frag_color;
layout (location = 1) out vec4 bloom_color;

in GS_OUT {
    vec4 color;
    vec2 texel;
} fs_in;

uniform sampler2D font;

void main() {
    float alpha = texture(font, fs_in.texel).r;
    if (alpha <= 0.0) {
        discard;
    }

    frag_color = fs_in.color;
    bloom_color = vec4(vec3(0.0), 1.0);
}