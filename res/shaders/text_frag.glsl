#version 330 core

out vec4 frag_color;

in VS_OUT {
    vec4 color;
    vec2 texel;
} fs_in;

uniform sampler2D font;

void main() {
    float alpha = texture(font, fs_in.texel).r;
    if (alpha <= 0.0) {
        discard;
    }
    frag_color = fs_in.color * vec4(vec3(1.0), alpha);
}