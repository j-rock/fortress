#version 330 core
layout (location = 0) out vec4 frag_color;
layout (location = 1) out vec4 bloom_color;

in GS_OUT {
    vec2 texel;
    vec4 rgba_color;
} fs_in;

uniform sampler2D font;

void main() {
    float font_intensity = texture(font, fs_in.texel).r;
    frag_color = vec4(fs_in.rgba_color.rgb, fs_in.rgba_color.a * font_intensity);
    bloom_color = vec4(vec3(0.0), 1.0);
}
