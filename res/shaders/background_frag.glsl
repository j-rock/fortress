#version 330 core
layout (location = 0) out vec4 frag_color;
layout (location = 1) out vec4 bloom_color;

in VS_OUT {
    vec2 texel;
} fs_in;

uniform sampler2D texture0;

void main() {
    vec4 srgb_alpha = texture(texture0, fs_in.texel);
    // Gamma correct.
    vec3 rgb = pow(srgb_alpha.rgb, vec3(1.0/2.2));
    frag_color = vec4(rgb, srgb_alpha.a);
    bloom_color = vec4(0.0, 0.0, 0.0, 1.0);
}